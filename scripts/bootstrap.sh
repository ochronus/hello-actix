#!/usr/bin/env sh
# Bootstrap script for hello-actix
# Sets up Rust toolchain and common tools on a fresh machine.
# Safe to run multiple times (idempotent best-effort).

set -eu

# -----------------------------
# Utilities
# -----------------------------

is_root() {
  [ "$(id -u)" -eq 0 ]
}

command_exists() {
  command -v "$1" > /dev/null 2>&1
}

run_sudo() {
  if is_root; then
    "$@"
  elif command_exists sudo; then
    sudo "$@"
  else
    echo "This action requires elevated privileges. Please re-run as root or install sudo."
    exit 1
  fi
}

ensure_path() {
  # Ensure cargo and local bin directories are on PATH for this session
  CARGO_BIN="$HOME/.cargo/bin"
  LOCAL_BIN="$HOME/.local/bin"

  case ":$PATH:" in
    *":$CARGO_BIN:"*) ;;
    *) export PATH="$CARGO_BIN:$PATH" ;;
  esac

  case ":$PATH:" in
    *":$LOCAL_BIN:"*) ;;
    *) export PATH="$LOCAL_BIN:$PATH" ;;
  esac
}

info() {
  printf "\033[1;34m[INFO]\033[0m %s\n" "$@"
}

warn() {
  printf "\033[1;33m[WARN]\033[0m %s\n" "$@"
}

error() {
  printf "\033[1;31m[ERROR]\033[0m %s\n" "$@" >&2
}

# -----------------------------
# Package manager helpers
# -----------------------------

detect_pm() {
  if command_exists apt-get; then
    echo apt
  elif command_exists dnf; then
    echo dnf
  elif command_exists pacman; then
    echo pacman
  elif command_exists zypper; then
    echo zypper
  elif command_exists brew; then
    echo brew
  else
    echo none
  fi
}

pm_install() {
  pm="$1"
  shift
  case "$pm" in
    apt)
      run_sudo apt-get update
      run_sudo env DEBIAN_FRONTEND=noninteractive apt-get install -y "$@"
      ;;
    dnf)
      run_sudo dnf install -y "$@"
      ;;
    pacman)
      run_sudo pacman -Sy --noconfirm "$@"
      ;;
    zypper)
      run_sudo zypper --non-interactive install -y "$@"
      ;;
    brew)
      # No sudo for brew
      brew install "$@" || true
      ;;
    *)
      warn "No supported package manager detected; skipping install of: $*"
      return 1
      ;;
  esac
}

# -----------------------------
# Rust toolchain
# -----------------------------

install_rust() {
  if command_exists rustup; then
    info "rustup already installed."
  else
    info "Installing rustup (Rust toolchain manager)..."
    # Non-interactive install for the current user
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  fi

  # Source cargo env if present (for current shell)
  if [ -f "$HOME/.cargo/env" ]; then
    # shellcheck disable=SC1090,SC1091
    # shellcheck source=/dev/null
    . "$HOME/.cargo/env"
  fi

  ensure_path

  info "Ensuring stable toolchain + components..."
  rustup toolchain install stable
  rustup default stable
  rustup component add clippy rustfmt
}

# -----------------------------
# System dependencies (OpenSSL + pkg-config)
# -----------------------------

install_system_deps() {
  pm="$(detect_pm)"
  info "Detected package manager: $pm"

  case "$pm" in
    apt)
      pm_install apt "libssl-dev pkg-config"
      ;;
    dnf)
      pm_install dnf "openssl-devel pkg-config"
      ;;
    pacman)
      pm_install pacman "openssl pkgconf"
      ;;
    zypper)
      pm_install zypper "libopenssl-devel pkg-config"
      ;;
    brew)
      pm_install brew "openssl@3 pkg-config"
      ;;
    none)
      warn "Could not detect a package manager. Please install OpenSSL headers and pkg-config manually."
      ;;
  esac
}

# -----------------------------
# Developer CLI tools
# -----------------------------

install_markdownlint() {
  if command_exists npm; then
    if npm -g ls markdownlint-cli > /dev/null 2>&1; then
      info "markdownlint-cli already installed."
    else
      info "Installing markdownlint-cli (global)..."
      npm install -g markdownlint-cli
    fi
  else
    warn "npm not found; skipping markdownlint-cli install."
  fi
}

install_actionlint() {
  if command_exists actionlint; then
    info "actionlint already installed."
    return
  fi

  pm="$(detect_pm)"
  case "$pm" in
    brew)
      info "Installing actionlint via brew..."
      brew install actionlint || true
      ;;
    *)
      info "Installing actionlint via upstream installer..."
      mkdir -p "$HOME/.local/bin"
      curl -sSfL https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash |
        bash -s -- -b "$HOME/.local/bin"
      ;;
  esac

  if command_exists actionlint; then
    info "actionlint installed."
  else
    warn "actionlint installation did not succeed; you can install it manually later."
  fi
}

install_shell_tools() {
  pm="$(detect_pm)"
  case "$pm" in
    apt | dnf | pacman | zypper | brew)
      info "Installing shellcheck and shfmt (best-effort)..."
      pm_install "$pm" "shellcheck" || warn "shellcheck not available via $pm"
      pm_install "$pm" "shfmt" || warn "shfmt not available via $pm"
      ;;
    none)
      warn "No package manager detected; skipping shellcheck/shfmt."
      ;;
  esac
}

install_cargo_tools() {
  ensure_path

  # Prefer cargo-binstall if available for faster binary installs
  if command_exists cargo-binstall; then
    info "Installing Rust CLI tools with cargo-binstall..."
    cargo binstall -y taplo-cli typos-cli cargo-audit cargo-deny cargo-outdated || true
  else
    warn "cargo-binstall not found; using 'cargo install --locked' (slower)."
    cargo install --locked taplo-cli typos-cli cargo-audit cargo-deny cargo-outdated || true
  fi
}

install_precommit() {
  if command_exists pre-commit; then
    info "pre-commit already installed."
    return
  fi

  if command_exists pipx; then
    info "Installing pre-commit via pipx..."
    pipx install pre-commit || true
  elif command_exists pip; then
    info "Installing pre-commit via pip (user)..."
    pip install --user pre-commit || true
  else
    warn "pip/pipx not found; skipping pre-commit installation."
  fi
}

# -----------------------------
# Main
# -----------------------------

main() {
  info "Bootstrapping development environment for hello-actix..."

  ensure_path
  install_rust
  install_system_deps
  install_cargo_tools
  install_markdownlint
  install_actionlint
  install_shell_tools
  install_precommit

  cat << 'EOT'

Bootstrap complete.

Next steps:
  - Ensure the following are on your PATH (add to your shell profile if needed):
      $HOME/.cargo/bin
      $HOME/.local/bin
  - Optionally enable pre-commit hooks:
      pre-commit install --hook-type pre-commit --hook-type pre-push
  - Run all checks locally:
      make lint
  - Or auto-format sources:
      make fix

Useful targets:
  - make tools        # (Re)install developer CLI tools
  - make bootstrap    # Similar flow from Makefile (you can use either)
  - make ci           # Run the same checks as CI locally

EOT
}

main "$@"
