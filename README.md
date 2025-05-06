# Sleep Timer with Pause

**指定時間に音楽を停止し、PCをスリープするクロスプラットフォームタイマー**

📢 [公式ダウンロードページはこちら](https://tororoMeshi.github.io/sleep-timer-gui/)

## 🖥 対応OS

- Windows 10 / 11
- Linux（Ubuntu, Debian, Fedora, Archなど AppImage 対応）

## 🔧 主な機能

- タイマー設定（○分後 / 指定時刻）
- 音楽停止（メディアキー送信）
- スリープ実行
- エラー処理（ポップアップ＆ログ）
- 前回設定の自動復元
- 自動起動（スタートアップ）設定
- タスクトレイ常駐

## 📦 ダウンロード

| OS     | ファイル                                    |
|--------|---------------------------------------------|
| Windows| [SleepTimerInstaller.exe](https://github.com/tororoMeshi/sleep-timer-gui/releases/latest/download/SleepTimerInstaller.exe) |
| Linux  | [SleepTimer-x86_64.AppImage](https://github.com/tororoMeshi/sleep-timer-gui/releases/latest/download/SleepTimer-x86_64.AppImage) |

## 📝 ビルド方法

**Windows**

```bash
cargo build --release
````

**Linux**

```bash
cargo install cargo-appimage
cargo appimage
```

## 📝 ログファイルの場所

| OS      | パス                             |
| ------- | ------------------------------ |
| Windows | `%APPDATA%\SleepTimer\log.txt` |
| Linux   | `~/.config/SleepTimer/log.txt` |

## 🧩 ライセンス

MIT

## 🙏 Special Thanks

* iced
* tray-icon
* anyhow
* simplelog
* rfd