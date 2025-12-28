# Makefile for hello-actix
# Common developer targets for linting, formatting, and tooling.

SHELL := /bin/sh

CARGO ?= cargo
TAPLO ?= taplo
TYPOS ?= typos
MDL ?= markdownlint
SHELLCHECK ?= shellcheck
SHFMT ?= shfmt

TOOLS_CARGO := taplo-cli typos-cli cargo-audit cargo-deny cargo-outdated

.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*## "}; /^[a-zA-Z0-9_.-]+:.*## / {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# --------------------------------------------------------------------------------------
# Formatting
# --------------------------------------------------------------------------------------

.PHONY: fmt
fmt: ## Format Rust code with rustfmt
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check: ## Check Rust formatting (no changes)
	$(CARGO) fmt --all -- --check

.PHONY: taplo
taplo: ## Format TOML files with Taplo
	$(TAPLO) fmt

.PHONY: taplo-check
taplo-check: ## Check TOML formatting (no changes)
	$(TAPLO) fmt --check

.PHONY: shfmt-fmt
shfmt-fmt: ## Format shell scripts with shfmt
	@if command -v $(SHFMT) >/dev/null 2>&1; then \
		$(SHFMT) -i 2 -ci -sr -w .; \
	else \
		echo "shfmt not found; skipping shell formatting. Install via your package manager (e.g., apt/brew)."; \
	fi

.PHONY: shfmt-check
shfmt-check: ## Check shell script formatting (no changes)
	@if command -v $(SHFMT) >/dev/null 2>&1; then \
		$(SHFMT) -i 2 -ci -sr -d .; \
	else \
		echo "shfmt not found; skipping shell formatting check. Install via your package manager (e.g., apt/brew)."; \
	fi

# --------------------------------------------------------------------------------------
# Linting / Analysis
# --------------------------------------------------------------------------------------

.PHONY: clippy
clippy: ## Run Clippy with warnings as errors
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: md-lint
md-lint: ## Lint Markdown files
	@if ! command -v $(MDL) >/dev/null 2>&1; then \
		echo "markdownlint not found; skipping. Run 'make tools' to install."; \
	else \
		files="$$(find . -type f -name '*.md' -not -path './target/*')"; \
		if [ -z "$$files" ]; then \
			echo "No Markdown files found."; \
		else \
			$(MDL) $$files; \
		fi; \
	fi

.PHONY: typos
typos: ## Spell-check code and docs with typos
	@if ! command -v $(TYPOS) >/dev/null 2>&1; then \
		echo "typos not found; skipping. Run 'make tools' to install."; \
	else \
		$(TYPOS); \
	fi

.PHONY: shellcheck
shellcheck: ## Static analysis for shell scripts
	@if ! command -v $(SHELLCHECK) >/dev/null 2>&1; then \
		echo "shellcheck not found; skipping. Run 'make tools' to install."; \
	else \
		files="$$(find . -type f -name '*.sh' -not -path './target/*')"; \
		if [ -z "$$files" ]; then \
			echo "No shell scripts found."; \
		else \
			$(SHELLCHECK) $$files; \
		fi; \
	fi

.PHONY: actionlint
actionlint: ## Lint GitHub Actions workflows (if installed)
	@if command -v actionlint >/dev/null 2>&1; then \
		actionlint; \
	else \
		echo "actionlint not found; install with 'brew install actionlint' (macOS) or see 'make tools'."; \
	fi

# --------------------------------------------------------------------------------------
# Security / Dependency policy
# --------------------------------------------------------------------------------------

.PHONY: audit
audit: ## Check for security advisories (RustSec)
	$(CARGO) audit

.PHONY: deny
deny: ## cargo-deny checks: advisories, bans, sources
	cargo deny check advisories
	cargo deny check bans
	cargo deny check sources

# --------------------------------------------------------------------------------------
# Aggregate targets
# --------------------------------------------------------------------------------------

.PHONY: fix
fix: fmt taplo shfmt-fmt ## Auto-format sources (Rust, TOML, shell)

.PHONY: lint
lint: pre-commit-run ## Run all pre-commit hooks

.PHONY: ci
ci: lint ## Alias for CI job locally

# --------------------------------------------------------------------------------------
# Tooling / Setup
# --------------------------------------------------------------------------------------

.PHONY: tools
tools: ## Install CLI tools (cargo-binstall if available, else cargo install). Also installs markdownlint-cli if npm is available.
	@if command -v cargo-binstall >/dev/null 2>&1; then \
		cargo binstall -y $(TOOLS_CARGO); \
	else \
		$(CARGO) install --locked $(TOOLS_CARGO); \
	fi
	@if command -v npm >/dev/null 2>&1; then \
		npm -g ls markdownlint-cli >/dev/null 2>&1 || npm install -g markdownlint-cli; \
	else \
		echo "npm not found; skipping markdownlint-cli install."; \
	fi
	@if command -v actionlint >/dev/null 2>&1; then \
		echo "actionlint already installed."; \
	elif command -v brew >/dev/null 2>&1; then \
		brew install actionlint || true; \
	else \
		curl -sSfL https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash | bash -s -- -b $$HOME/.local/bin; \
		echo "Ensure $$HOME/.local/bin is on your PATH"; \
	fi

.PHONY: system-deps
system-deps: ## Install system dependencies for native-tls/OpenSSL (best-effort)
	@if command -v apt-get >/dev/null 2>&1; then \
		sudo apt-get update && sudo apt-get install -y libssl-dev pkg-config; \
	elif command -v dnf >/dev/null 2>&1; then \
		sudo dnf install -y openssl-devel pkg-config; \
	elif command -v pacman >/dev/null 2>&1; then \
		sudo pacman -S --noconfirm openssl pkgconf; \
	elif command -v brew >/dev/null 2>&1; then \
		brew install openssl@3 pkg-config; \
	else \
		echo "Please install OpenSSL development headers and pkg-config for your OS."; \
	fi

.PHONY: bootstrap
bootstrap: ## Install Rust toolchain, system dependencies, and CLI tools
	@rustup show >/dev/null 2>&1 || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	rustup toolchain install stable
	rustup default stable
	rustup component add clippy rustfmt
	$(MAKE) system-deps
	$(MAKE) tools

# --------------------------------------------------------------------------------------
# Misc
# --------------------------------------------------------------------------------------

.PHONY: trailing-whitespace-check
trailing-whitespace-check: ## Fail on trailing whitespace in tracked text files
	@set -e; \
	if command -v git >/dev/null 2>&1; then \
	  files="$$(git ls-files)"; \
	else \
	  files="$$(find . -type f -not -path './.git/*' -not -path './target/*')"; \
	fi; \
	status=0; \
	for f in $$files; do \
	  grep -I -n -E "[[:blank:]]+$$" "$$f" >/dev/null && { echo "Trailing whitespace: $$f"; status=1; }; \
	done; \
	exit $$status

.PHONY: eof-check
eof-check: ## Fail if tracked text files don't end with a newline
	@set -e; \
	if command -v git >/dev/null 2>&1; then \
	  files="$$(git ls-files)"; \
	else \
	  files="$$(find . -type f -not -path './.git/*' -not -path './target/*')"; \
	fi; \
	status=0; \
	for f in $$files; do \
	  [ -s "$$f" ] || continue; \
	  last=$$(tail -c1 "$$f" | od -An -t u1 | tr -d ' '); \
	  if [ "$$last" != "10" ]; then \
	    echo "Missing trailing newline at EOF: $$f"; \
	    status=1; \
	  fi; \
	done; \
	exit $$status

.PHONY: pre-commit-run
pre-commit-run: ## Run pre-commit hooks (required)
	@if command -v pre-commit >/dev/null 2>&1; then \
		pre-commit run --all-files --show-diff-on-failure; \
	else \
		echo "pre-commit not found. Install via 'make tools' (pipx/pip) and retry."; \
		exit 1; \
	fi

.PHONY: clean
clean: ## Clean cargo artifacts
	$(CARGO) clean
