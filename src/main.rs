mod tray;
mod log;
mod autostart;

use chrono::{Duration as ChronoDuration, Local, NaiveTime, Timelike};
use iced::executor;
use iced::theme::Theme;
use iced::widget::{button, column, row, text, text_input, checkbox};
use iced::{Alignment, Application, Command, Element, Settings, Subscription};
use iced::time::{self, Duration};
use anyhow::{Error, Context};
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::process::Command as SysCommand;

fn execute_actions() -> Result<(), Error> {
    #[cfg(target_os = "linux")]
    {
        log::info_popup("動作", "メディアキー（Linux）を送信しています");
        let pause = SysCommand::new("xdotool")
            .arg("key")
            .arg("XF86AudioPlay")
            .status()
            .context("xdotool 呼び出し失敗")?;
        if !pause.success() {
            return Err(anyhow::anyhow!("xdotool 実行失敗"));
        }

        std::thread::sleep(Duration::from_secs(2));

        log::info_popup("動作", "スリープ（Linux）を実行しています");
        let sleep = SysCommand::new("systemctl")
            .arg("suspend")
            .status()
            .context("systemctl suspend 呼び出し失敗")?;
        if !sleep.success() {
            return Err(anyhow::anyhow!("systemctl suspend 実行失敗"));
        }
    }

    #[cfg(target_os = "windows")]
    {
        use winapi::um::winuser::{
            INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VK_MEDIA_PLAY_PAUSE,
        };
        use std::mem::size_of;

        log::info_popup("動作", "メディアキー（Windows）を送信しています");

        unsafe {
            let mut input_down = INPUT {
                type_: INPUT_KEYBOARD,
                u: std::mem::zeroed(),
            };
            *input_down.u.ki_mut() = KEYBDINPUT {
                wVk: VK_MEDIA_PLAY_PAUSE as u16,
                wScan: 0,
                dwFlags: 0,
                time: 0,
                dwExtraInfo: 0,
            };

            let mut input_up = INPUT {
                type_: INPUT_KEYBOARD,
                u: std::mem::zeroed(),
            };
            *input_up.u.ki_mut() = KEYBDINPUT {
                wVk: VK_MEDIA_PLAY_PAUSE as u16,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            };

            let inputs = [input_down, input_up];
            let sent = SendInput(
                inputs.len() as u32,
                inputs.as_ptr() as *mut INPUT,
                size_of::<INPUT>() as i32,
            );

            if sent != inputs.len() as u32 {
                return Err(anyhow::anyhow!("メディアキー送信に失敗しました"));
            }
        }

        std::thread::sleep(Duration::from_secs(2));

        log::info_popup("動作", "スリープ（Windows）を実行しています");
        use winapi::um::powrprof::SetSuspendState;
        unsafe {
            SetSuspendState(0, 0, 0);
        }
    }

    Ok(())
}

#[derive(Default)]
struct TimerApp {
    mode: Mode,
    input_value: String,
    status: String,
    target_time: Option<NaiveTime>,
    auto_start: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
enum Mode {
    #[default]
    AfterMinutes,
    AtTime,
}

#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    mode: Mode,
    last_input: String,
    auto_start: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ModeChanged(Mode),
    InputChanged(String),
    AutoStartChanged(bool),
    Start,
    Tick,
}

impl Application for TimerApp {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        if let Err(e) = log::init_logging() {
            log::report_error("ログ初期化エラー", &e);
        }
        if let Err(e) = tray::create_tray() {
            log::report_error("タスクトレイ初期化エラー", &e);
        }
        let config = load_config().unwrap_or_default();
        (
            TimerApp {
                mode: config.mode,
                input_value: config.last_input,
                status: String::new(),
                target_time: None,
                auto_start: config.auto_start,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Sleep Timer with Pause")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ModeChanged(mode) => {
                self.mode = mode;
                self.status.clear();
                self.target_time = None;
            }
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::AutoStartChanged(v) => {
                self.auto_start = v;
                let result = if v {
                    autostart::enable_autostart()
                } else {
                    autostart::disable_autostart()
                };
                if let Err(e) = result {
                    log::report_error("自動起動設定エラー", &e);
                } else {
                    log::info_popup("自動起動設定", if v { "有効化しました" } else { "無効化しました" });
                }
                let config = AppConfig {
                    mode: self.mode,
                    last_input: self.input_value.clone(),
                    auto_start: self.auto_start,
                };
                if let Err(e) = save_config(&config) {
                    log::report_error("設定保存エラー", &e);
                }
            }
            Message::Start => {
                let now = Local::now().time();
                let result = (|| -> Result<(), Error> {
                    let target = if self.mode == Mode::AfterMinutes {
                        let minutes: u32 = self.input_value.trim().parse()
                            .context("分数の入力が不正です")?;
                        let t = (now + ChronoDuration::minutes(minutes as i64))
                            .with_second(0).context("秒の丸め処理エラー")?;
                        Ok(t)
                    } else {
                        let t = NaiveTime::parse_from_str(&self.input_value, "%H:%M")
                            .context("時刻の形式が不正です（HH:MM）")?;
                        Ok(t)
                    }?;

                    self.target_time = Some(target);
                    self.status = format!("タイマーセット: {}", target.format("%H:%M"));

                    let config = AppConfig {
                        mode: self.mode,
                        last_input: self.input_value.clone(),
                        auto_start: self.auto_start,
                    };
                    save_config(&config)?;

                    Ok(())
                })();

                if let Err(e) = result {
                    log::report_error("タイマー開始時のエラー", &e);
                    self.status = "エラー: 入力を確認してください".to_string();
                }
            }
            Message::Tick => {
                if let Some(target) = self.target_time {
                    let now = Local::now().time();
                    if now >= target {
                        self.status = "時間になりました！ 動作実行中...".to_string();
                        self.target_time = None;
                        match execute_actions() {
                            Ok(_) => {
                                self.status = "動作成功（音楽停止＋スリープ）".to_string();
                            }
                            Err(e) => {
                                log::report_error("アクション実行エラー", &e);
                                self.status = "動作中にエラーが発生しました".to_string();
                            }
                        }
                    } else {
                        let remaining = (target.num_seconds_from_midnight() as i64
                            - now.num_seconds_from_midnight() as i64)
                            .max(0);
                        let minutes = remaining / 60;
                        let seconds = remaining % 60;
                        self.status = format!(
                            "残り: {}分{}秒（終了予定: {}）",
                            minutes,
                            seconds,
                            target.format("%H:%M")
                        );
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mode_buttons = row![
            button("○分後").on_press(Message::ModeChanged(Mode::AfterMinutes)),
            button("HH:MM").on_press(Message::ModeChanged(Mode::AtTime))
        ]
        .spacing(10);

        let input = text_input(
            match self.mode {
                Mode::AfterMinutes => "分数を入力",
                Mode::AtTime => "HH:MM形式で入力",
            },
            &self.input_value,
            Message::InputChanged,
        );

        let auto_start_checkbox = checkbox(
            "起動時に自動起動",
            self.auto_start,
            Message::AutoStartChanged,
        );

        let start_button = button("タイマー開始").on_press(Message::Start);

        let content = column![
            mode_buttons,
            input,
            auto_start_checkbox,
            start_button,
            text(&self.status)
        ]
        .padding(20)
        .spacing(10)
        .align_items(Alignment::Center);

        content.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}

fn load_config() -> Result<AppConfig, Error> {
    let path = config_path()?;
    let data = std::fs::read_to_string(path)?;
    let config: AppConfig = serde_json::from_str(&data)?;
    Ok(config)
}

fn save_config(config: &AppConfig) -> Result<(), Error> {
    let path = config_path()?;
    let data = serde_json::to_string_pretty(config)?;
    std::fs::write(path, data)?;
    Ok(())
}

fn config_path() -> Result<PathBuf, Error> {
    let proj_dirs = ProjectDirs::from("com", "YourName", "SleepTimer")
        .ok_or_else(|| anyhow::anyhow!("設定ディレクトリ取得失敗"))?;
    let dir = proj_dirs.config_dir();
    std::fs::create_dir_all(dir)?;
    Ok(dir.join("config.json"))
}

fn main() -> iced::Result {
    TimerApp::run(Settings {
        window: iced::window::Settings {
            size: (400, 250),
            ..Default::default()
        },
        ..Default::default()
    })
}
