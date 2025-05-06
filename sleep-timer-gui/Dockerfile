FROM rust:1.86-slim

# 必要なツールをインストール
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev git curl && \
    cargo install cargo-release && \
    rm -rf /var/lib/apt/lists/*

# 作業ディレクトリ
WORKDIR /app
