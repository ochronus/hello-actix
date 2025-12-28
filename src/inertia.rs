#![allow(clippy::module_name_repetitions)]

// Inertia + Vite initialization helpers for this project, mirroring the `actix_ssr` example.
// - Uses Vite HBS template resolver (root template at `www/root.hbs`).
// - Publishes an assets version via OnceLock that is derived from the Vite hash.
// - Enables SSR with a local SSR client.
//
// Typical usage in main (example):
//   let inertia = inertia::initialize_inertia().await?;
//   let inertia = web::Data::new(inertia);
//   HttpServer::new(move || App::new().app_data(inertia.clone()) /* ... */)

use std::{io, sync::OnceLock};

use inertia_rust::{
    Inertia, InertiaConfig, InertiaError, InertiaVersion, SsrClient,
    template_resolvers::ViteHBSTemplateResolver,
};
use vite_rust::{Vite, ViteConfig, ViteMode};

/// Global assets version derived from Vite's current hash.
///
/// Set once during Vite initialization; used by Inertia for cache-busting.
pub static ASSETS_VERSION: OnceLock<&str> = OnceLock::new();
pub static DEV_MODE: OnceLock<bool> = OnceLock::new();

/// Initialize and configure Vite for this project.
///
/// Configuration matches the example app:
/// - Manifest path: `public/bundle/manifest.json`
/// - Entrypoints: `www/app.tsx`, `www/index.css`
/// - Prefix: `/bundle`
///
/// Also sets [`ASSETS_VERSION`] using Vite's hash (or "development" as a fallback).
pub async fn initialize_vite() -> Vite {
    let vite_config = ViteConfig::default()
        .set_manifest_path("public/bundle/manifest.json")
        // Allow development without a manifest when the Vite dev server is running.
        .set_entrypoints(vec!["www/app.tsx", "www/index.css"])
        // Prefix every asset path with "bundle" so preloads work correctly.
        .set_prefix("/bundle");

    match Vite::new(vite_config).await {
        Err(err) => panic!("{err}"),
        Ok(vite) => {
            let _ = ASSETS_VERSION.set(vite.get_hash().unwrap_or("development").to_string().leak());
            vite
        }
    }
}

/// Initialize Inertia with Vite + Handlebars template resolver and SSR enabled.
///
/// - Root HBS template: `www/root.hbs`
/// - SSR client: 127.0.0.1:1000 (expects Node SSR server started by Inertia manager)
/// - Base URL is derived from `PORT` or `APP_PORT` env vars (defaults to 1337)
pub async fn initialize_inertia() -> Result<Inertia, io::Error> {
    let vite = initialize_vite().await;
    let dev_mode = *vite.mode() == ViteMode::Development;
    let _ = DEV_MODE.set(dev_mode);

    let resolver = ViteHBSTemplateResolver::builder()
        .set_vite(vite)
        .set_template_path("www/root.hbs")
        .set_dev_mode(dev_mode)
        .build()
        .map_err(InertiaError::to_io_error)?;

    // Derive base URL from environment (matches this project's default port logic).
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .or_else(|| std::env::var("APP_PORT").ok().and_then(|s| s.parse::<u16>().ok()))
        .unwrap_or(1337);
    let base_url = format!("http://localhost:{port}");
    let base_url: &'static str = Box::leak(base_url.into_boxed_str());

    let version = InertiaVersion::Literal(ASSETS_VERSION.get().copied().unwrap_or("development"));

    // Determine whether to enable SSR:
    // - Only enable in production mode
    // - Do NOT enable when Vite dev server is active
    let app_mode =
        std::env::var("APP_MODE").or_else(|_| std::env::var("APP__MODE")).unwrap_or_default();
    let is_prod_mode = matches!(app_mode.to_ascii_lowercase().as_str(), "prod" | "production");
    let mut enable_ssr = is_prod_mode && !dev_mode;
    // Optional global toggle: INERTIA_SSR
    // - "off" | "false" | "0"   => disable SSR even in prod
    // - "on"  | "true"  | "1"   => force-enable SSR (not recommended in dev)
    if let Ok(v) = std::env::var("INERTIA_SSR") {
        match v.to_ascii_lowercase().as_str() {
            "off" | "false" | "0" => enable_ssr = false,
            "on" | "true" | "1" => enable_ssr = true,
            _ => {}
        }
    }

    let cfg_builder = InertiaConfig::builder()
        .set_url(base_url)
        .set_version(version)
        .set_template_resolver(Box::new(resolver));

    // Use a fixed SSR port to keep alignment with the SSR entry server
    let ssr_port = 5174;

    let cfg = if enable_ssr {
        cfg_builder
            .enable_ssr()
            // The SSR Node server will be spawned separately by the Inertia manager,
            // and this client must match the configured host/port.
            .set_ssr_client(SsrClient::new("127.0.0.1", ssr_port))
            .build()
    } else {
        cfg_builder.build()
    };

    Inertia::new(cfg)
}
