//! Typed application configuration built on top of the `config` crate.
//!
//! Sources (in increasing precedence):
//! - Optional files: `config/default.{toml|yaml|json|ini}`, `config/local.{...}`
//! - Environment variables with prefix `APP` and `__` as a separator (e.g. `APP_PORT=8080`)
//! - Legacy `PORT` environment variable as a last override for `port`
//!
//! Fields:
//! - `port` (u16): TCP port to bind, defaults to 1337
//! - `secret_key` (SecretKey): 512-bit cookie key; by default it is generated at startup
//! - `cookie_name` (String): session cookie name, defaults to "auth-example"
//! - `cookie_ttl_seconds` (u64): cookie/session TTL in seconds, defaults to 300 (5 minutes)
//! - `mode` (RuntimeMode): dev|prod|test (default: prod). In prod, cookies are marked secure.
//!
//! Environment examples:
//!   APP_PORT=8080
//!   APP_COOKIE_NAME=my-session
//!   APP_COOKIE_TTL_SECONDS=600
//!   APP_MODE=dev
//!   APP_SECRET_KEY=base64:3vT3...   # base64 string (>= 64 bytes after decode), see notes below
//!
//! Secret key notes:
//! - Prefer specifying a stable key via APP_SECRET_KEY in production.
//! - Supported formats: "base64:<...>", "hex:<...>", or raw (first try base64, then hex).
//! - If omitted, a random key is generated at startup (okay for dev/test; not recommended for prod).
//!
//! Usage in main:
//!   let cfg = AppConfig::load()?;
//!   let port = cfg.effective_port();
//!   let secret_key = cfg.secret_key.clone_key();
//!   let cookie_secure = cfg.cookie_secure();
//!   let ttl = cfg.cookie_ttl();
//!   let cookie_name = cfg.cookie_name.clone();

use std::fmt;
use std::ops::Deref;

use actix_web::cookie::{Key, time::Duration as CookieDuration};
use base64::{Engine as _, engine::general_purpose};
use config as cfg;
use serde::Deserialize;

/// Runtime mode for the service.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeMode {
    Dev,
    #[default]
    Prod,
    Test,
}

impl fmt::Display for RuntimeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RuntimeMode::Dev => "dev",
            RuntimeMode::Prod => "prod",
            RuntimeMode::Test => "test",
        };
        f.write_str(s)
    }
}

/// Newtype wrapper around `actix_web::cookie::Key` with custom deserialization.
///
/// Accepts one of:
/// - "base64:<ENCODED_BYTES>"  (recommended)
/// - "hex:<HEX_BYTES>"
/// - raw string (attempts base64 first, then hex)
///
/// If not provided, defaults to a randomly generated key (NOT recommended for prod).
#[derive(Clone)]
pub struct SecretKey(pub Key);

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't leak key material in logs.
        f.debug_tuple("SecretKey").field(&"<redacted>").finish()
    }
}

impl SecretKey {
    /// Clone out the inner `Key`.
    pub fn clone_key(&self) -> Key {
        self.0.clone()
    }
}

impl Default for SecretKey {
    fn default() -> Self {
        SecretKey(Key::generate())
    }
}

impl Deref for SecretKey {
    type Target = Key;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SecretKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SecretKeyVisitor;

        impl<'de> serde::de::Visitor<'de> for SecretKeyVisitor {
            type Value = SecretKey;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a base64 or hex encoded secret key string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let s = v.trim();
                if s.is_empty() || s.eq_ignore_ascii_case("generate") {
                    return Ok(SecretKey(Key::generate()));
                }

                // Helper to try construct a Key from bytes.
                let try_from_bytes = |bytes: &[u8]| -> Result<SecretKey, E> {
                    use std::convert::TryFrom;
                    Key::try_from(bytes)
                        .map(SecretKey)
                        .map_err(|_| E::custom("secret key must be >= 64 bytes after decoding"))
                };

                // Detect explicit prefixes
                if let Some(b64) = s.strip_prefix("base64:") {
                    let decoded = general_purpose::STANDARD
                        .decode(b64)
                        .map_err(|e| E::custom(format!("invalid base64 secret key: {e}")))?;
                    return try_from_bytes(&decoded);
                }

                if let Some(hex) = s.strip_prefix("hex:") {
                    let decoded = hex_decode(hex)
                        .map_err(|e| E::custom(format!("invalid hex secret key: {e}")))?;
                    return try_from_bytes(&decoded);
                }

                // Try base64 first, then hex without prefix
                if let Ok(decoded) = general_purpose::STANDARD.decode(s) {
                    return try_from_bytes(&decoded);
                }
                if let Ok(decoded) = hex_decode(s) {
                    return try_from_bytes(&decoded);
                }

                Err(E::custom(
                    "secret key must be provided as base64:<...> or hex:<...> (>= 64 bytes)",
                ))
            }
        }

        deserializer.deserialize_any(SecretKeyVisitor)
    }
}

/// Simple hex decoder (lower/upper mixed) without adding an extra crate.
///
/// Returns an error if length is odd or any char is not hex.
fn hex_decode(input: &str) -> Result<Vec<u8>, String> {
    let s = input.trim();
    if !s.len().is_multiple_of(2) {
        return Err("hex string must have an even length".into());
    }
    fn val(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    }
    let bytes = s
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let hi =
                val(pair[0]).ok_or_else(|| format!("invalid hex char: {}", char::from(pair[0])))?;
            let lo =
                val(pair[1]).ok_or_else(|| format!("invalid hex char: {}", char::from(pair[1])))?;
            Ok((hi << 4) | lo)
        })
        .collect::<Result<Vec<u8>, String>>()?;
    Ok(bytes)
}

/// Strongly-typed application configuration.
#[derive(Clone, Deserialize)]
pub struct AppConfig {
    /// TCP port to bind. Default: 1337
    #[serde(default = "AppConfig::default_port")]
    pub port: u16,

    /// Cookie/session secret key. See `SecretKey` docs. Default: randomly generated.
    #[serde(default)]
    pub secret_key: SecretKey,

    /// Session cookie name. Default: "auth-example"
    #[serde(default = "AppConfig::default_cookie_name")]
    pub cookie_name: String,

    /// Cookie/session TTL in seconds. Default: 300 (5 minutes)
    #[serde(default = "AppConfig::default_cookie_ttl_seconds")]
    pub cookie_ttl_seconds: u64,

    /// Runtime mode: dev|prod|test. Default: prod
    #[serde(default)]
    pub mode: RuntimeMode,
}

impl AppConfig {
    fn default_port() -> u16 {
        1337
    }

    fn default_cookie_name() -> String {
        "auth-example".to_owned()
    }

    fn default_cookie_ttl_seconds() -> u64 {
        5 * 60
    }

    /// Load configuration from files, environment and `.env`.
    ///
    /// Files checked (all optional):
    /// - `config/default.*`
    /// - `config/local.*`
    ///
    /// Environment:
    /// - Prefixed with `APP` and `__` separator, e.g.:
    ///   - `APP_PORT=8080`
    ///   - `APP_COOKIE_NAME=my-session`
    ///   - `APP_COOKIE_TTL_SECONDS=600`
    ///   - `APP_MODE=dev`
    ///   - `APP_SECRET_KEY=base64:...`
    ///
    /// Legacy override:
    /// - `PORT` environment variable (common in PaaS) overrides `port` if present.
    pub fn load() -> Result<Self, cfg::ConfigError> {
        // Load environment variables from .env if present
        let _ = dotenvy::dotenv();

        let builder = cfg::Config::builder()
            .add_source(cfg::Environment::with_prefix("APP").separator("__").try_parsing(true));

        let mut conf: AppConfig = builder.build()?.try_deserialize()?;

        // Honor legacy `PORT` as an override if set
        if let Ok(port_s) = std::env::var("PORT")
            && let Ok(p) = port_s.parse::<u16>()
        {
            conf.port = p;
        }

        Ok(conf)
    }

    /// Whether cookies should be marked `Secure` under this mode.
    /// - In `prod`: true
    /// - In `dev`/`test`: false
    pub fn cookie_secure(&self) -> bool {
        matches!(self.mode, RuntimeMode::Prod)
    }

    /// Return TTL as an Actix cookie duration.
    pub fn cookie_ttl(&self) -> CookieDuration {
        // Saturate i64 if someone sets a huge value.
        let secs_i64 = self.cookie_ttl_seconds.min(i64::MAX as u64) as i64;
        CookieDuration::seconds(secs_i64)
    }

    /// Compute the effective port, honoring `PORT` env override applied in `load()`.
    pub fn effective_port(&self) -> u16 {
        self.port
    }
}
