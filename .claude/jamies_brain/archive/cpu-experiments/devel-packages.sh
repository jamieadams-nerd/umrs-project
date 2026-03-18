#!/usr/bin/env bash
#
# UMRS
# For Ubuntu Development syste
set -euo pipefail

if [[ ${EUID:-$(id -u)} -eq 0 ]]; then
  echo "Do not run as root."
  exit 1
fi

if ! command -v sudo >/dev/null 2>&1; then
  echo "sudo not found."
  exit 1
fi

sudo apt update

sudo apt install -y build-essential clang cmake make pkg-config git curl wget unzip zip tar jq tree ripgrep fd-find htop tmux rsync gnupg ca-certificates software-properties-common python3 python3-pip python3-venv python3-dev libssl-dev openssl asciidoc neovim

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

# shellcheck disable=SC1090
source "$HOME/.cargo/env"

rustup default stable
rustup component add rustfmt clippy >/dev/null 2>&1 || true

if ! command -v node >/dev/null 2>&1; then
  curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
  sudo apt install -y nodejs
fi

command -v prettier >/dev/null 2>&1 || sudo npm install -g prettier
command -v antora   >/dev/null 2>&1 || sudo npm install -g @antora/cli @antora/site-generator

BASHRC="$HOME/.bashrc"
add_line() { grep -qxF "$1" "$2" 2>/dev/null || echo "$1" >> "$2"; }

add_line "export EDITOR=nvim" "$BASHRC"
add_line "export VISUAL=nvim" "$BASHRC"
add_line "export CARGO_TERM_COLOR=always" "$BASHRC"
add_line "export RUST_BACKTRACE=1" "$BASHRC"

echo "OK: installed base packages, rustup+rustfmt+clippy, nodejs+prettier, antora, nvim."
echo "Open a new shell (or run: source ~/.bashrc) to pick up env changes."

