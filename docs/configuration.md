# Configuration

This document describes how application configuration works, what sources are
used, the available settings, and recommended practices for development and
production.

## Overview

- Configuration is strongly typed and deserialized into an `AppConfig` struct at
  startup.
- Sources:
  - A `.env` file (loaded if present).
  - Environment variables with prefix `APP` and `__` as a separator (for nested
    keys).
  - Legacy `PORT` environment variable (overrides `port` if present).
- No additional config files are used (e.g., TOML/YAML) — only `.env` and
  environment variables.
- The configuration object is registered in the web framework’s application
  state so it can be accessed from any request handler.

## Settings schema (typed)

- `APP_PORT` (u16)
  - Description: TCP port the server binds to.
  - Default: `1337`
- `APP_SECRET_KEY` (SecretKey)
  - Description: Session/cookie encryption and signing key.
  - Default: generated randomly at startup (acceptable for dev/test; not
    recommended for prod).
  - Format:
    - `base64:<BASE64_STRING>` (recommended)
    - `hex:<HEX_STRING>`
    - Raw base64/hex (without prefix) is also accepted but discouraged in favor
      of explicit prefixes.
  - Minimum decoded length: 64 bytes (512 bits).
- `APP_COOKIE_NAME` (String)
  - Description: Name of the session cookie.
  - Default: `auth-example`
- `APP_COOKIE_TTL_SECONDS` (u64)
  - Description: Session/cookie TTL in seconds.
  - Default: `300` (5 minutes)
- `APP_MODE` (RuntimeMode)
  - Description: Runtime mode for behavioral flags.
  - Values: `prod` (default), `dev`, `test`
  - Cookie security:
    - `prod`: cookies are marked `Secure` (sent only over HTTPS)
    - `dev` and `test`: cookies are NOT marked `Secure` (to allow local HTTP)

Additional override:

- `PORT`
  - If set, overrides `APP_PORT`. This is commonly provided by PaaS
    environments (e.g., Heroku/Fly.io).

## Source precedence

1. `.env` is loaded into the process environment (if present).
1. Environment variables prefixed with `APP__` are parsed into the typed config
   (e.g., `APP__COOKIE_NAME` -> `cookie_name`). For simplicity, you can also use
   single underscore: `APP_COOKIE_NAME`, which is the common convention for flat
   keys.
1. Legacy `PORT` overrides the computed `port` (if set and parseable as a
   `u16`).

Net effect: You can manage all configuration via `.env` locally, while
production/deployment can provide real environment variables to override any
values.

## Example .env

- `APP_MODE=prod` — default. Use `dev`/`test` locally when you want non-secure
  cookies.
- `APP_PORT=1337` — set the bind port explicitly (or rely on `PORT`).
- `APP_COOKIE_NAME=auth-example`
- `APP_COOKIE_TTL_SECONDS=300`
- `APP_SECRET_KEY=base64:...` — REQUIRED for production. Must decode to at
  least 64 bytes.

Tip: To generate a secure base64 key (Linux/macOS):

- `openssl rand -base64 64`
- or `head -c 64 /dev/urandom | base64`

Hex alternative:

- `openssl rand -hex 64`

## Mapping between env and fields

- `APP_PORT` -> `port`
- `APP_SECRET_KEY` -> `secret_key`
- `APP_COOKIE_NAME` -> `cookie_name`
- `APP_COOKIE_TTL_SECONDS` -> `cookie_ttl_seconds`
- `APP_MODE` -> `mode`

Special:

- `PORT` -> overrides `port` if present.

## Behavior in the web server

- The application config is constructed at startup and registered in the
  application state.
- Session middleware uses:
  - `secret_key` (cloned per worker) to sign/encrypt cookies.
  - `cookie_name` for the cookie name.
  - `cookie_ttl_seconds` to set the persistent session lifetime.
  - `mode` to decide whether the cookie is `Secure`.

This ensures that changing configuration automatically adjusts middleware
behavior without code changes.

## Security recommendations

- Always set a stable `APP_SECRET_KEY` in production. Do not rely on the
  default random key in prod — it will invalidate all sessions across restarts.
- Use `APP_MODE=prod` behind HTTPS so cookies are marked `Secure`.
- Keep `.env` gitignored (already configured) and provide a `.env.example` for
  documentation and onboarding.
- Rotate the secret key with care if necessary, understanding that active
  sessions may be invalidated.

## Local development tips

- Use `APP_MODE=dev` (or `test`) when you need to use cookies over plain HTTP on
  localhost.
- For most local usage, letting the application generate a random
  `APP_SECRET_KEY` is fine. When you need stable sessions across restarts, set a
  persistent dev key in your `.env`.

## Operations and deployment

- Prefer environment variables (`APP_*`) injected by your orchestrator (Docker,
  Kubernetes, systemd) over `.env` in production.
- `PORT` is honored if your PaaS injects it. You can set both `PORT` and
  `APP_PORT`, but `PORT` wins if both are present.

## Troubleshooting

- Invalid secret key:
  - Ensure it’s base64 or hex and decodes to at least 64 bytes.
  - Prefer explicit prefixes: `base64:` or `hex:`.
- Cookies not being sent:
  - In `prod` mode, cookies are `Secure`; they require HTTPS.
  - In `dev/test` mode, cookies are not `Secure` and will work over HTTP.
- Session not persisting:
  - Check `APP_COOKIE_TTL_SECONDS`.
  - Ensure the cookie name is consistent and not clashing with other middleware
    or proxies.
