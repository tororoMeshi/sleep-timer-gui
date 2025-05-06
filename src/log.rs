use anyhow::Error;
use simplelog::*;
use std::fs::File;
use std::path::PathBuf;
use directories::ProjectDirs;
use rfd::MessageDialog;

pub fn init_logging() -> Result<(), Error> {
    let log_file = log_path()?;
    let file = File::create(&log_file)?;
    WriteLogger::init(LevelFilter::Info, Config::default(), file)
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

pub fn report_error(message: &str, error: &Error) {
    let log_message = format!("{}: {:?}", message, error);
    log::error!("{}", log_message);

    MessageDialog::new()
        .set_title("エラー")
        .set_description(format!("{}\n詳細: {:?}", message, error))
        .show();
}

pub fn info_popup(title: &str, message: &str) {
    log::info!("{}: {}", title, message);
    MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .show();
}

fn log_path() -> Result<PathBuf, Error> {
    let proj_dirs = ProjectDirs::from("com", "YourName", "SleepTimer")
        .ok_or_else(|| anyhow::anyhow!("ログディレクトリ取得失敗"))?;
    let dir = proj_dirs.config_dir();
    std::fs::create_dir_all(dir)?;
    Ok(dir.join("log.txt"))
}
