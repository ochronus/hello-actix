# Architecture

This document explains the structure, data flow, and key design choices of the
application. It’s written to help you navigate the codebase and extend it
safely.

## Overview

- Web framework: Actix Web (async, multi-worker HTTP server).
- Session & identity: actix-session (cookie store) and actix-identity.
- Configuration: typed, loaded from `.env` and environment variables into
  `AppConfig`.
- Dependency injection: Actix’s application state via `web::Data<T>`.
- Handlers: organized under `src/handlers`, registered via a central
  `init` function.

## Project layout (relevant parts)

- `src/main.rs`
  - Bootstraps the app: loads typed config, sets up middleware, registers
    routes, and starts the HTTP server.
- `src/config.rs`
  - Strongly typed configuration with serde and the `config` crate.
  - Secret key deserializer (base64/hex) for secure cookie operations.
- `src/handlers/`
  - `mod.rs`: aggregates handler modules and exposes `init(cfg)` to register
    them.
  - `index.rs`, `login.rs`, `logout.rs`, `echo.rs`: individual route handlers.
- `docs/`
  - `configuration.md`: how configuration is loaded and used.
  - `architecture.md`: this document.

## Request lifecycle

1. A TCP connection is accepted by Actix’s server and assigned to one of N
   workers.
1. The request enters the middleware pipeline (identity, session) before
   reaching the route.
1. The handler executes, possibly extracting shared state (e.g.,
   `web::Data<AppConfig>`) and request-specific extractors
   (path/query/body/session/identity).
1. The handler produces a type that implements `Responder`, which is converted
   into an `HttpResponse`.
1. The response flows back through middleware (e.g., setting session cookies).

## Configuration

See `docs/configuration.md` for details.

Highlights:

- Loaded at startup via `AppConfig::load()`. Sources:
  - `.env` (if present)
  - Env vars prefixed with `APP_` (typed)
  - Legacy `PORT` env var override (commonly set by PaaS)
- Mode controls cookie security:
  - `prod` (default): cookies are `Secure`
  - `dev`/`test`: cookies are not `Secure` (allows HTTP during local
    development)
- The configuration is registered as shared app state (`web::Data<AppConfig>`)
  for extraction in handlers.

## Dependency injection and shared state

- The idiomatic Actix approach is to register shared, immutable state with
  `app_data(web::Data::new(state))`.
- Handlers receive `web::Data<AppConfig>` as an argument to access
  configuration.
- This pattern is ergonomic, test-friendly (you can build a test app with test
  config), and thread-safe.

Examples of what belongs in app state:

- `AppConfig`
- Database connection pools
- Caches or clients wrapped in `Arc` if needed

## Middleware and session management

- `IdentityMiddleware`: Enables attaching a verified identity to a session
  (e.g., after login).
- `SessionMiddleware` with `CookieSessionStore`:
  - Uses the configured secret key to sign and encrypt session cookies.
  - Cookie name and TTL are configurable.
  - `cookie_secure` is driven by the runtime mode (`prod` vs `dev`/`test`).

Rationale:

- Cookie-backed sessions are simple to operate and sufficient for many apps.
- The secret key must be stable in production; otherwise, all sessions
  invalidate on restart.

## Server bootstrap and thread model

- The server is created with `HttpServer::new(move || App::new()... )`. The
  `move` closure “owns” what it captures so it can be sent to worker threads
  safely.
- Actix spawns multiple worker threads (by default, the number of logical CPU
  cores).
- Each worker has its own `App` instance. Shared state (e.g., config) is
  provided by cloning `web::Data` handles, which are cheap (internally
  reference-counted).

Key implications:

- Anything captured by the factory closure must be `'static` and `Send`.
- Prefer `Arc` (or `web::Data`) for shared state to avoid unnecessary cloning.

## Handlers organization

- Each route handler lives in its own module/file under `src/handlers`.
- `src/handlers/mod.rs` exposes an `init(cfg: &mut web::ServiceConfig)` that
  registers all routes.
- Handlers can access configuration (and other shared state) by adding
  `web::Data<AppConfig>` to the function parameters.
- This separation keeps `main.rs` focused on composition and startup, while
  business logic lives in handlers.

## Error handling

- Startup errors (e.g., config errors) are mapped to `std::io::Error` for a
  simple exit path.
- Handlers currently return simple responses for clarity. As the app grows:
  - Consider a custom error type that implements `ResponseError` for consistent
    HTTP errors.
  - Add contextual logging at error boundaries.

## Security considerations

- Secret key:
  - Must decode to at least 64 bytes (512 bits).
  - Use base64 or hex (explicit prefixes are preferred: `base64:` or `hex:`).
  - Always set a stable key in production; do not rely on random generation
    there.
- Cookie security:
  - In `prod`, cookies are `Secure` and require HTTPS.
  - In `dev/test`, cookies are not `Secure`, easing local development over
    HTTP.
- Session TTL:
  - Choose a TTL that balances user experience with security.
- Logging:
  - Use `RUST_LOG` to tune logging. Avoid logging secrets or PII.

## Observability

- Enable logs with `RUST_LOG=info,actix_web=info` (or finer-grained tuning).
- Consider integrating structured logging (e.g., `tracing`) and request IDs for
  production.
- Health probe endpoints (e.g., `/health`) are easy to add later for deployment
  readiness checks.

## Extensibility

- Adding a database:
  - Initialize a connection pool in `main.rs` and add it to app state
    (`web::Data<Pool>`).
  - In handlers, extract `web::Data<Pool>` alongside `web::Data<AppConfig>`.
- Adding more middleware:
  - Chain with `.wrap(...)` in the app factory; order matters
    (authentication, logging, etc.).
- Versioned APIs:
  - Split handlers into submodules by API version (`handlers/v1`,
    `handlers/v2`) and configure them conditionally.

## Testing

- Unit test handlers by constructing a test `App` with `actix_web::test`,
  injecting a test `AppConfig` and any additional state.
- For session/identity tests:
  - Use the same middleware stack in the test app to simulate real behavior.
  - Control `APP_MODE` for secure vs non-secure cookie behavior.

## Trade-offs and choices

- `web::Data` for DI:
  - Idiomatic in Actix; straightforward and type-safe.
  - Avoids global mutable state and keeps tests ergonomic.
- Cookie session store:
  - Simpler operationally than Redis/DB-backed sessions.
  - If you need server-side invalidation or very large session payloads,
    consider a server-side store.
- Minimalistic error handling initially:
  - Keeps the learning curve low.
  - Easy to evolve into richer error types and responses later.

---

If you’re adding a new feature and unsure where it belongs:

- Configuration? Add fields to `AppConfig` and document them in
  `docs/configuration.md`.
- Cross-cutting concern? Consider middleware or app state.
- A new route? Add a handler under `src/handlers` and register it in
  `handlers::init`.

This architecture favors clarity, testability, and gradual evolution as the
application grows.
