# Repository Guidelines

## プロジェクト構成とモジュール
`src/main.rs` がアプリ全体のエントリーポイントで、タイマー画面、状態管理、設定ファイルのロードを統括します。`src/tray.rs` はタスクトレイ常駐とメニュー処理を担い、`src/autostart.rs` は Linux デスクトップ向けの autostart `.desktop` を管理します。`src/log.rs` は簡易ログ機能を提供し、配布用アイコンは `assets/icon.png`、公開サイトとドキュメントは `docs/index.md`、変更履歴は `CHANGELOG.md` にまとまっています。AppImage ビルド手順は `sleep-timer-gui/` 内の Dockerfile と `run-cargo-release.sh` に整理済みです。ビルド成果物は `target/`、設定・ログは実行時に `~/.config/SleepTimer/` へ出力される点に注意してください。

## ビルド・テスト・開発コマンド
Rust stable toolchain を使用し、開発中は `cargo run` で挙動を確認します。最適化ビルドは `cargo build --release`、AppImage 配布物は `cargo install cargo-appimage` 後に `cargo appimage` で生成し、成果物は `target/appimage/` に配置されます。品質担保のため `cargo test`、`cargo clippy -- -D warnings`、`cargo fmt -- --check` を CI 同様にローカルで実行し、必要に応じて `cargo update` や `cargo tree -d` で依存関係を確認してください。

## コーディングスタイルと命名規約
Rust 2021 edition を前提とし、`rustfmt.toml` が無いため標準 rustfmt ルール（4 スペースインデント、連鎖メソッドは改行）に従います。モジュール・関数・変数は `snake_case`、型とトレイトは `UpperCamelCase`、定数は `SCREAMING_SNAKE_CASE` を使用します。ユーザーへ表示する文言とログメッセージは既存実装同様に自然な日本語を心掛け、`anyhow::Result` を経由したエラー伝播と `log::{info,warn,error}` のレベル選択を明示してください。OS 依存コードは `cfg` 属性で分岐し、モジュール毎に責務が混在しないよう短い関数へ分割します。

## テストガイドライン
UI ロジックからビジネスロジックを切り出し、モジュール内に `#[cfg(test)] mod tests` を用意して境界値（0 分、複数時間、日付跨ぎ）を網羅します。外部 API に依存する箇所はラッパーを導入し、モック可能なインターフェースを介して検証してください。将来的に統合テストを追加する場合は `tests/` ディレクトリにシナリオ別ファイル（例: `tests/sleep_timer_behaviour.rs`）を配置し、ファイル冒頭で前提条件をコメントしてください。PR 前には `cargo test`、必要に応じて `cargo run --release` で実行確認し、失敗時は原因と再現手順を記録します。

## コミットとプルリクエスト
コミットメッセージは Git 履歴に倣い、先頭を動詞または名詞句で始めた 50 文字以内の要約（例: `タイマー設定の永続化を修正`）とし、本文に背景・検証結果・互換性へ影響を追記します。作業ブランチは `feature/<topic>` や `fix/<issue-id>` のように用途を明確に命名してください。Pull Request では目的、主要変更点、テスト結果（実行したコマンドと要約）、スクリーンショットやログの抜粋を添付し、関連 Issue や外部チケットをリンクします。レビューに備え、`cargo fmt` と `cargo clippy` を通し、コンフリクトを解消した状態でドラフトから Ready へ切り替えてください。

## リリースと配布フロー
タグ `v*` を push すると GitHub Actions が Linux 用 AppImage をビルドし、自動でリリースに添付します。ローカルで同等ビルドを再現する際は `sleep-timer-gui/run-cargo-release.sh --clean` を利用し、`--no-cache` オプションで Docker イメージを更新できます。配布前には `target/appimage/` の実行確認と `CHANGELOG.md`・`docs/index.md` のバージョン表記更新を行い、タグ作成前に成果物を最終確認してください。リリース後は GitHub Actions のログでエラーがないか確認し、問題があればタグを修正して再実行します。

## 環境設定とサポート
Rust と cargo は `rustup` でインストールし、Ubuntu では `libappindicator3` や `libwebkit2gtk-4.1-dev` などトレイ表示・GUI に必要なネイティブライブラリを事前に導入してください。開発用ログは `~/.config/SleepTimer/log.txt` に蓄積されるため、問題報告時には最新 200 行程度を添付すると解析が円滑です。GUI の翻訳やキー送信に関する挙動は issue に記録し、再現条件・環境・期待動作を明確に書くだけでサポートが迅速になります。開発用テンプレートや追加スクリプトを作成した場合は `docs/` に概要を追記し、再利用できるよう共有してください。
