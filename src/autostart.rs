use anyhow::{Context, Error};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub fn enable_autostart() -> Result<(), Error> {
    use std::env;
    use std::fs;
    let exe_path = env::current_exe().context("exeパス取得失敗")?;
    let startup_dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("スタートアップディレクトリ取得失敗"))?
        .join("Microsoft/Windows/Start Menu/Programs/Startup");

    fs::create_dir_all(&startup_dir)?;
    let shortcut = startup_dir.join("SleepTimer.lnk");
    fs::copy(&exe_path, &shortcut)
        .context("スタートアップ用ファイル作成失敗")?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn disable_autostart() -> Result<(), Error> {
    let startup_dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("スタートアップディレクトリ取得失敗"))?
        .join("Microsoft/Windows/Start Menu/Programs/Startup");
    let shortcut = startup_dir.join("SleepTimer.lnk");

    if shortcut.exists() {
        std::fs::remove_file(&shortcut)?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn enable_autostart() -> Result<(), Error> {
    let autostart_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("autostartディレクトリ取得失敗"))?
        .join("autostart");
    std::fs::create_dir_all(&autostart_dir)?;

    let desktop_entry = autostart_dir.join("sleep_timer.desktop");
    let exe_path = std::env::current_exe().context("exeパス取得失敗")?;

    let content = format!(
        "[Desktop Entry]\nType=Application\nName=Sleep Timer\nExec=\"{}\"\n",
        exe_path.display()
    );

    std::fs::write(&desktop_entry, content)
        .context("autostart用.desktopファイル作成失敗")?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn disable_autostart() -> Result<(), Error> {
    let desktop_entry = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("autostartディレクトリ取得失敗"))?
        .join("autostart/sleep_timer.desktop");

    if desktop_entry.exists() {
        std::fs::remove_file(&desktop_entry)?;
    }
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn enable_autostart() -> Result<(), Error> {
    Err(anyhow::anyhow!("このOSは自動起動未対応"))
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn disable_autostart() -> Result<(), Error> {
    Err(anyhow::anyhow!("このOSは自動起動未対応"))
}
