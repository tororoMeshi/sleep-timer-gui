#!/usr/bin/env bash
# WSL2 + WSLg で Rust/Tauri GUI を動かすための依存チェック & 起動スクリプト
# 使い方:
#   chmod +x wsl-gui-check.sh
#   ./wsl-gui-check.sh            # 依存チェック → 必要ならインストール → GUI起動
#   ./wsl-gui-check.sh --run-only # インストールせずに起動だけ
#   ./wsl-gui-check.sh --deps     # 依存導入のみ（起動しない）

set -euo pipefail

PROJECT_NAME="${PROJECT_NAME:-$(basename "$(pwd)")}"
RUN_ONLY=false
INSTALL_DEPS_ONLY=false

for arg in "$@"; do
  case "$arg" in
    --run-only) RUN_ONLY=true ;;
    --deps) INSTALL_DEPS_ONLY=true ;;
    *) echo "Unknown arg: $arg" >&2; exit 2 ;;
  esac
done

info(){ printf "\033[1;36m[INFO]\033[0m %s\n" "$*"; }
warn(){ printf "\033[1;33m[WARN]\033[0m %s\n" "$*"; }
err(){  printf "\033[1;31m[ERR ]\033[0m %s\n" "$*"; }

# -------------------------------
# 0) 実行環境: WSL/WSLg の検出
# -------------------------------
detect_wsl(){
  if grep -qi "microsoft" /proc/sys/kernel/osrelease 2>/dev/null || [ -n "${WSL_DISTRO_NAME-}" ]; then
    info "WSL 環境を検出: ${WSL_DISTRO_NAME:-unknown}"
  else
    err "WSL 上で実行されていません。Windows 上の WSL2 ターミナルで実行してください。"
    exit 1
  fi
}

show_wsl_versions(){
  # 可能なら Windows の wsl.exe からバージョン表示
  if command -v wsl.exe >/dev/null 2>&1; then
    info "WSL バージョン情報:"
    wsl.exe --version || true
  else
    warn "wsl.exe が見つかりません（古い WSL かも）。PowerShell で 'wsl --version' を実行して確認してください。"
  fi
}

detect_wslg(){
  local ok=0
  # WSLg が有効な場合、/mnt/wslg や WAYLAND_DISPLAY, DISPLAY, PULSE 等がある
  if [ -d /mnt/wslg ]; then ok=1; fi
  if [ -n "${WAYLAND_DISPLAY-}" ]; then ok=1; fi
  if [ -n "${DISPLAY-}" ]; then ok=1; fi
  if [ -S /mnt/wslg/PulseServer ]; then ok=1; fi

  if [ "$ok" -eq 1 ]; then
    info "WSLg (GUI サポート) を検出しました。"
  else
    warn "WSLg を検出できませんでした。GUI 表示に失敗する可能性があります。"
    warn "PowerShell（管理者）で以下を実行し、WSL を最新化して再起動してください："
    printf "  wsl --update\n  wsl --shutdown\n"
  fi
}

# -------------------------------
# 1) パッケージ・ツールの導入
# -------------------------------
need_install(){
  # dpkg -s で有無判断
  local pkg="$1"
  dpkg -s "$pkg" >/dev/null 2>&1 || return 0
  return 1
}

apt_install_if_missing(){
  local to_install=()
  for p in "$@"; do
    if need_install "$p"; then to_install+=("$p"); fi
  done
  if [ "${#to_install[@]}" -gt 0 ]; then
    info "APT で導入します: ${to_install[*]}"
    sudo apt-get update -y
    sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "${to_install[@]}"
  else
    info "必要な APT パッケージは揃っています。"
  fi
}

ensure_node20(){
  if command -v node >/dev/null 2>&1; then
    info "Node.js 検出: $(node -v)"
    return
  fi
  info "Node.js が見つかりません。Node 20 LTS を導入します。"
  curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
  sudo apt-get update -y
  sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends nodejs
  info "Node.js: $(node -v), npm: $(npm -v)"
}

ensure_rust(){
  if command -v rustc >/dev/null 2>&1; then
    info "Rust 検出: $(rustc --version)"
  else
    warn "Rust が見つかりません。rustup で導入します。"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # shellcheck disable=SC1091
    source "$HOME/.cargo/env"
    info "Rust: $(rustc --version)"
  fi
}

ensure_tauri_cli_if_needed(){
  if [ -d "src-tauri" ]; then
    if command -v cargo-tauri >/dev/null 2>&1 || cargo tauri --help >/dev/null 2>&1; then
      info "tauri-cli 検出済み。"
    else
      info "tauri-cli を導入します（cargo install tauri-cli）"
      cargo install tauri-cli
    fi
  fi
}

install_gui_deps(){
  # 共通依存
  local common=(
    libgtk-3-dev
    libayatana-appindicator3-dev
    librsvg2-dev
    build-essential
    pkg-config
    curl
    ca-certificates
    git
    python3
  )

  # WebKitGTK を自動選択（24.04 は 4.1 or 6.0、22.04 は 4.0）
  local webkit_pkg=""
  if apt-cache show libwebkit2gtk-4.1-dev >/dev/null 2>&1; then
    webkit_pkg="libwebkit2gtk-4.1-dev"
  elif apt-cache show libwebkitgtk-6.0-dev >/dev/null 2>&1; then
    webkit_pkg="libwebkitgtk-6.0-dev"
  elif apt-cache show libwebkit2gtk-4.0-dev >/dev/null 2>&1; then
    webkit_pkg="libwebkit2gtk-4.0-dev"
  else
    echo "[ERR ] WebKitGTK の dev パッケージが見つかりませんでした。" >&2
    echo "       Ubuntu 24.04 なら tauri v2 を推奨、または Docker/22.04 でビルドしてください。" >&2
    exit 1
  fi

  info "APT で導入します: ${webkit_pkg} ${common[*]}"
  sudo apt-get update -y
  sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    "$webkit_pkg" "${common[@]}"
}


# -------------------------------
# 2) 実行ターゲットの判定と起動
# -------------------------------
run_app(){
  if [ -d "src-tauri" ]; then
    info "Tauri プロジェクトと判断（src-tauri/ が存在）。GUI を起動します。"
    info "コマンド: cargo tauri dev"
    cargo tauri dev
  else
    info "通常の Rust GUI/CLI と判断。cargo run を実行します。"
    info "コマンド: cargo run"
    cargo run
  fi
}

# -------------------------------
# メイン処理
# -------------------------------
detect_wsl
show_wsl_versions
detect_wslg

if [ "${RUN_ONLY}" = false ]; then
  install_gui_deps
  ensure_node20
  ensure_rust
  ensure_tauri_cli_if_needed
fi

if [ "${INSTALL_DEPS_ONLY}" = true ]; then
  info "依存導入のみ実施しました。起動は行いません。"
  exit 0
fi

run_app
