use anyhow::{Context, Error};
use tray_icon::{
    menu::{Menu, MenuItem},
    TrayIconBuilder,
};
use std::sync::Arc;

pub fn create_tray() -> Result<(), Error> {
    let quit = MenuItem::new("終了", true, |_| {
        std::process::exit(0);
    });

    let menu = Menu::new().append(&quit);

    let _tray_icon = TrayIconBuilder::new()
        .sender_waker(Arc::new(tray_icon::Waker::new()))
        .tooltip("Sleep Timer")
        .icon(tray_icon::Icon::from_file_data(include_bytes!("../icon.png"), None)
            .context("トレイアイコン読み込み失敗")?)
        .menu(Box::new(menu))
        .build()
        .context("タスクトレイ作成失敗")?;

    // TrayIconBuilderの戻り値は使用しないが、アプリのライフタイムに存在する必要がある。
    std::mem::forget(_tray_icon);

    Ok(())
}
