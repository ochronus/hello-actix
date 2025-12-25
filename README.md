# hello-actix

A minimal Actix Web application with session middleware and a production-ready
developer workflow:

- Linting and formatting
- Dependency and security checks
- Consistent editor/formatting configs
- GitHub Actions CI and Dependabot

Endpoints:

- GET / → “Hello world!”
- POST /echo → echoes the request body
- GET /hey → “Hey there!”

The server binds to 127.0.0.1:1337 by default and generates a random cookie key
at startup. This setup is for local/dev use (cookie secure = false).

---

## Quickstart

1. Clone and bootstrap

- This installs Rust, system dependencies (OpenSSL+pkg-config), and all dev tools.

```sh
sh scripts/bootstrap.sh
```

1. Run the server

```sh
cargo run
```

1. Verify

- GET [http://127.0.0.1:1337/](http://127.0.0.1:1337/)
- POST [http://127.0.0.1:1337/echo](http://127.0.0.1:1337/echo) with any body
- GET [http://127.0.0.1:1337/hey](http://127.0.0.1:1337/hey)

---

## Requirements

- Rust (via rustup) with components:
  - clippy
  - rustfmt
- System libraries for native-tls/OpenSSL
  - Linux (Debian/Ubuntu): libssl-dev, pkg-config
  - Fedora: openssl-devel, pkg-config
  - Arch: openssl, pkgconf
  - macOS: openssl@3, pkg-config (Homebrew)
- Optional tools (installed by bootstrap/Makefile):
  - taplo-cli, typos-cli, cargo-audit, cargo-deny, cargo-outdated
  - markdownlint-cli (via npm)
  - shellcheck, shfmt
  - actionlint
  - pre-commit

If you prefer to install manually, see the “Manual setup” section below.

---

## Tooling overview

- rustfmt: opinionated Rust formatter
- Clippy: Rust lints (CI runs with -D warnings)
- Taplo: TOML formatter and schema validation
- markdownlint: Markdown style checks
- typos: fast spell checker across code and docs
- cargo-audit: RustSec security advisories
- cargo-deny: policy checks for advisories, duplicate crates, and sources
  - License checks are intentionally disabled here
- shellcheck: shell script static analysis
- shfmt: shell script formatter
- actionlint: lints GitHub Actions workflows
- pre-commit: local hooks runner (optional, but recommended)

Configs:

- .clippy.toml
- rustfmt.toml
- taplo.toml
- .markdownlint.json
- typos.toml
- deny.toml (without license policy)
- .editorconfig

---

## Makefile targets

The Makefile provides convenient, composable tasks.

Formatting:

- fmt — format Rust
- fmt-check — check Rust formatting
- taplo — format TOML files
- taplo-check — check TOML formatting
- shfmt-fmt — format shell scripts
- shfmt-check — check shell formatting

Linting/analysis:

- clippy — run Clippy with -D warnings
- md-lint — lint Markdown
- typos — spell-check code and docs
- shellcheck — static analysis for shell scripts
- actionlint — lint GitHub Actions (if installed)

Security/deps:

- audit — cargo-audit (RustSec)
- deny — cargo-deny checks (advisories, bans, sources)

Aggregates:

- fix — auto-format Rust, TOML, and shell scripts
- lint — run all checks (format checks, lints, security, policy)
- ci — alias for lint

Tooling/setup:

- tools — install all developer CLI tools (uses cargo-binstall if present;
  otherwise cargo install; installs markdownlint-cli via npm if available)
- system-deps — attempt to install OpenSSL headers and pkg-config for your OS
- bootstrap — install Rust toolchain, system dependencies, and tools (similar to
  the bootstrap script)

Misc:

- clean — cargo clean

Examples:

- Run all checks:

```sh
make lint
```

- Auto-format everything:

```sh
make fix
```

- Install/refresh developer tools:

```sh
make tools
```

---

## CI

GitHub Actions workflows:

- .github/workflows/ci.yml
  - Matrix job: clippy and rustfmt
  - Installs libssl-dev/pkg-config on Linux runners for native-tls
  - Uses the stable Rust toolchain with clippy and rustfmt components
  - Caches cargo builds
- .github/workflows/quality.yml
  - taplo (TOML), markdownlint, typos
  - shellcheck, shfmt
  - cargo-audit
  - cargo-deny: advisories, bans, sources (license check removed)
  - actionlint

Dependabot:

- .github/dependabot.yml
  - Daily updates for Cargo and GitHub Actions (no PR limit)

Run the same checks locally:

- Equivalent to CI:

```sh
make ci
```

---

## Manual setup (alternative to scripts/bootstrap.sh)

1. Rust and components:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install stable
rustup default stable
rustup component add clippy rustfmt
```

1. System dependencies:

- Debian/Ubuntu:

```sh
sudo apt-get update && sudo apt-get install -y libssl-dev pkg-config
```

- Fedora:

```sh
sudo dnf install -y openssl-devel pkg-config
```

- Arch:

```sh
sudo pacman -S --noconfirm openssl pkgconf
```

- macOS (Homebrew):

```sh
brew install openssl@3 pkg-config
```

If needed on macOS:

```sh
export PKG_CONFIG_PATH="$(brew --prefix)/opt/openssl@3/lib/pkgconfig"
```

1. Developer CLIs:

- Faster (recommended), if you have cargo-binstall:

```sh
cargo install cargo-binstall
cargo binstall taplo-cli typos-cli cargo-audit cargo-deny cargo-outdated -y
```

- Portable:

```sh
cargo install taplo-cli typos-cli cargo-audit cargo-deny cargo-outdated --locked
```

- Markdown:

```sh
npm install -g markdownlint-cli
```

- Shell tools:
  - Linux: use your package manager to install shellcheck and shfmt
  - macOS:

```sh
brew install shellcheck shfmt
```

- actionlint:
  - macOS:

```sh
brew install actionlint
```

- actionlint (Portable): see the [project page](https://github.com/rhysd/actionlint).

1. Optional pre-commit hooks:

```sh
pipx install pre-commit   # or: pip install --user pre-commit
pre-commit install --hook-type pre-commit --hook-type pre-push
```

---

## Project structure

- src/main.rs — Actix Web app with session middleware and a few routes
- Cargo.toml — package manifest
- rustfmt.toml — Rust formatter config (stable options)
- .clippy.toml — Clippy thresholds and allowances (tests are pragmatic)
- taplo.toml — TOML formatter and schema validation
- .markdownlint.json — Markdown rules
- typos.toml — spell checker config
- deny.toml — cargo-deny configuration (advisories, bans, sources)
- .editorconfig — unified editor settings
- .github/workflows/ci.yml — clippy + rustfmt
- .github/workflows/quality.yml — extended quality/security pipeline
- .github/dependabot.yml — daily dependency updates
- scripts/bootstrap.sh — one-shot setup on a fresh machine
- Makefile — targets for formatting, linting, security, and setup

---

## App notes

- Session middleware uses a random key generated at runtime and cookie_secure = false
  for local development.
- Not suitable for production without revisiting session storage, key management,
  TLS, and other security considerations.
- Listening address/port: 127.0.0.1:1337

---

## Troubleshooting

- OpenSSL/pkg-config not found on macOS:
  - Ensure you installed with Homebrew and exported
    PKG_CONFIG_PATH if needed:
    - export PKG_CONFIG_PATH="$(brew --prefix)/opt/openssl@3/lib/pkgconfig"
- npm not found:
  - Skip markdownlint or install Node.js (apt/brew) first.
- actionlint not found:
  - Use Homebrew on macOS or fetch the released binary via the upstream
    install script.
- Slow tool installation via cargo:
  - Consider installing cargo-binstall to speed up CLI installs.

---

## Contributing

- Keep PRs small and focused.
- Run local checks before pushing:

```sh
make lint
```

- Prefer fixing lints over disabling them. If you must allow something, document
  the rationale with a comment or in the PR description.

