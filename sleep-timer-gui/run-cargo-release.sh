#!/bin/bash
set -eux

# sleep-timer-gui プロジェクトの現在ディレクトリ
PROJECT_DIR=$(pwd)

# イメージビルド
docker build -t rust-cargo-release .

# コンテナ内で cargo release 実行（タグ作成＆push）
docker run --rm \
    -v "$PROJECT_DIR":/app \
    -w /app \
    -e CARGO_TERM_COLOR=always \
    rust-cargo-release \
    cargo release patch
