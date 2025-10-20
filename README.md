# Sleep Timer with Pause

**指定時間に音楽を停止し、PCをスリープするクロスプラットフォームタイマー**

📢 [公式ダウンロードページはこちら](https://tororoMeshi.github.io/sleep-timer-gui/)

## 🖥 対応OS

- Linux（Ubuntu）

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
| Linux  | [SleepTimer-x86_64.AppImage](https://github.com/tororoMeshi/sleep-timer-gui/releases/latest/download/SleepTimer-x86_64.AppImage) |

## 📝 ビルド方法

**Linux**

```bash
cargo install cargo-appimage
cargo appimage
```

## 🧪 Lint を Docker で実行

ローカル環境を汚さずに検証する場合は、`scripts/docker-lint.sh` を利用してください。

```bash
./scripts/docker-lint.sh
```

環境変数 `EXTRA_COMMAND` で追加コマンドを連結できます。例: `EXTRA_COMMAND="cargo test" ./scripts/docker-lint.sh`
Ubuntu 向けビルドのみを想定しているため、Windows 向けターゲットはチェック対象に含まれません。

## 🏗 Docker でリリースビルド

リリースバイナリや AppImage をコンテナ上で生成する場合は、`scripts/docker-build.sh` を使用します。

```bash
./scripts/docker-build.sh
```

`SKIP_APPIMAGE=1` を指定すると `cargo appimage` をスキップできます。追加コマンドは `EXTRA_COMMAND` で連結可能です。AppImage 生成時はコンテナ内で `appimagetool` をダウンロードし、`APPIMAGE_EXTRACT_AND_RUN=1` を設定するためホスト側に FUSE を準備する必要はありません。

## 📝 ログファイルの場所

| OS      | パス                             |
| ------- | ------------------------------ |
| Linux   | `~/.config/SleepTimer/log.txt` |

## 🧩 ライセンス

MIT
