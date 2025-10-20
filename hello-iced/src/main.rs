use iced::time;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::widget::text::Style as TextStyle;
use iced::{application, Element, Length, Subscription, Task, Theme};
use iced::alignment::{Horizontal, Vertical};
use iced::Color;
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    HoursChanged(String),
    MinutesChanged(String),
    StartPressed,
    Tick,
}

struct State {
    hours_input: String,
    minutes_input: String,
    remaining_seconds: Option<u32>,
    finished: bool,
    error: Option<String>,
}

fn initialize() -> (State, Task<Message>) {
    (
        State {
            hours_input: "0".into(),
            minutes_input: "1".into(),
            remaining_seconds: None,
            finished: false,
            error: None,
        },
        Task::none(),
    )
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::HoursChanged(value) => {
            state.hours_input = value;
            state.error = None;
        }
        Message::MinutesChanged(value) => {
            state.minutes_input = value;
            state.error = None;
        }
        Message::StartPressed => match parse_total_seconds(&state.hours_input, &state.minutes_input) {
            Ok(total) if total > 0 => {
                state.remaining_seconds = Some(total);
                state.finished = false;
                state.error = None;
            }
            Ok(_) => {
                state.error = Some("Please enter a duration greater than zero.".into());
            }
            Err(msg) => {
                state.error = Some(msg);
            }
        },
        Message::Tick => {
            if let Some(remaining) = state.remaining_seconds {
                if remaining > 1 {
                    state.remaining_seconds = Some(remaining - 1);
                } else {
                    state.remaining_seconds = None;
                    state.finished = true;

                    match trigger_system_sleep() {
                        Ok(()) => state.error = None,
                        Err(msg) => state.error = Some(msg),
                    }
                }
            }
        }
    }

    Task::none()
}

fn view(state: &State) -> Element<Message> {
    let timer_label = if state.finished {
        "Time is up!".to_owned()
    } else if let Some(remaining) = state.remaining_seconds {
        format_remaining(remaining)
    } else {
        "Set a duration and press Start".to_owned()
    };

    let inputs = row![
        column![
            text("Hours"),
            text_input("0", &state.hours_input).on_input(Message::HoursChanged)
        ],
        column![
            text("Minutes"),
            text_input("0", &state.minutes_input).on_input(Message::MinutesChanged)
        ]
    ]
    .spacing(20);

    let start_label = container(text("Start").size(24))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    let start_button = button(start_label)
        .width(Length::Fixed(180.0))
        .height(Length::Fixed(52.0))
        .padding([12, 24])
        .style(|theme, status| {
            let mut style = button::primary(theme, status);
            style.text_color = Color::WHITE;
            style
        })
        .on_press(Message::StartPressed);

    let header = container(text(timer_label).size(36))
        .width(Length::Fill)
        .height(Length::Fixed(56.0))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    let mut content = column![
        header,
        inputs,
        Space::with_height(Length::Fill),
        start_button
    ]
    .spacing(20)
    .padding(20);

    if let Some(msg) = &state.error {
        content = content.push(
            text(msg)
                .style(|_| TextStyle {
                    color: Some(Color::from_rgb(0.9, 0.2, 0.2)),
                    ..Default::default()
                })
                .height(Length::Fixed(24.0)),
        );
    }

    content.into()
}

fn subscription(state: &State) -> Subscription<Message> {
    if state.remaining_seconds.is_some() && !state.finished {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    } else {
        Subscription::none()
    }
}

fn theme(_: &State) -> Theme {
    Theme::Dark
}

fn main() -> iced::Result {
    application("Simple Timer", update, view)
        .subscription(subscription)
        .theme(theme)
        .window_size([420.0, 260.0])
        .centered()
        .run_with(initialize)
}

fn parse_total_seconds(hours: &str, minutes: &str) -> Result<u32, String> {
    let hours_value: u32 = hours
        .trim()
        .parse()
        .map_err(|_| "Hours must be a non-negative integer.".to_owned())?;
    let minutes_value: u32 = minutes
        .trim()
        .parse()
        .map_err(|_| "Minutes must be a non-negative integer.".to_owned())?;

    if minutes_value >= 60 {
        return Err("Minutes must be less than 60.".into());
    }

    Ok(hours_value * 3600 + minutes_value * 60)
}

fn format_remaining(total_seconds: u32) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02} remaining", hours, minutes, seconds)
}

fn trigger_system_sleep() -> Result<(), String> {
    let status = Command::new("systemctl")
        .arg("suspend")
        .status()
        .map_err(|e| format!("Failed to execute systemctl suspend: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("systemctl suspend exited with status: {status}"))
    }
}
