use std::sync::Arc;

use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{App, HttpServer, web};
use inertia_rust::{InertiaProp, actix::InertiaMiddleware, hashmap};

mod config;
mod handlers;
mod inertia;

async fn manual_hello() -> &'static str {
    "Hey there!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize env and logging
    let _ = dotenvy::dotenv();
    env_logger::init();

    // Load typed configuration (reads .env and environment)
    let cfg: config::AppConfig =
        config::AppConfig::load().map_err(|e| std::io::Error::other(e.to_string()))?;
    println!("Runtime mode (effective): {}", cfg.mode);
    let bind_port = cfg.effective_port();
    let cfg_data: web::Data<config::AppConfig> = web::Data::new(cfg);

    // Initialize Inertia manager (Vite + HBS resolver, SSR enabled)
    let inertia_manager = inertia::initialize_inertia().await?;
    let inertia_data = web::Data::new(inertia_manager);
    let inertia_clone = inertia_data.clone();
    let vite_dev = crate::inertia::DEV_MODE.get().copied().unwrap_or(false);
    println!("Vite dev mode detected: {}", vite_dev);

    // Build and bind the server
    let server = HttpServer::new({
        let cfg_data = cfg_data.clone();
        move || {
            let cfg = cfg_data.get_ref();
            App::new()
                .app_data(cfg_data.clone())
                .app_data(inertia_clone.clone())
                // Identity + Session (cookie-based)
                .wrap(IdentityMiddleware::default())
                .wrap(
                    SessionMiddleware::builder(
                        // [TLA+ Invariant] Safety: Session Integrity
                        // We use CookieSessionStore which generates unique random IDs.
                        // This satisfies the TLA+ requirement that Login picks a "Fresh" ID
                        // (not currently in use).
                        CookieSessionStore::default(),
                        cfg.secret_key.clone_key(),
                    )
                    .cookie_name(cfg.cookie_name.clone())
                    .cookie_secure(cfg.cookie_secure())
                    .session_lifecycle(PersistentSession::default().session_ttl(cfg.cookie_ttl()))
                    .build(),
                )
                // Inertia middleware with shared props (assets version for cache-busting)
                .wrap(InertiaMiddleware::new().with_shared_props(Arc::new(|_req| {
                    Box::pin(async {
                        hashmap![
                            "assetsVersion" => InertiaProp::always(
                                crate::inertia::ASSETS_VERSION
                                    .get()
                                    .copied()
                                    .unwrap_or("development")
                            )
                        ]
                    })
                })))
                // App routes
                .configure(handlers::init)
                .route("/hey", web::get().to(manual_hello))
                // Serve static files from ./public (e.g., /bundle/* from Vite)
                // Keep this last so it doesn't shadow other routes.
                .service(actix_files::Files::new("/", "./public/").prefer_utf8(true))
        }
    })
    .bind(("0.0.0.0", bind_port))?;

    // Start the Node SSR process only in production. In dev/test we rely on Vite dev server and skip SSR silently.
    let is_prod = matches!(cfg_data.get_ref().mode, config::RuntimeMode::Prod);
    let node = if is_prod && !vite_dev {
        println!("Starting SSR server on port 5174...");
        let ssr_entry = "dist/ssr/ssr.js";
        if std::path::Path::new(ssr_entry).exists() {
            match inertia_data.start_node_server(ssr_entry.into()) {
                Ok(n) => Some(n),
                Err(e) => {
                    eprintln!(
                        "Inertia SSR not started: failed to launch SSR server at {ssr_entry}: {e}"
                    );
                    None
                }
            }
        } else {
            // In production, warn if the SSR bundle is missing.
            eprintln!(
                "Inertia SSR not started: {ssr_entry} not found. Run `npm run build` to generate it."
            );
            None
        }
    } else {
        // Dev/test or Vite dev server detected: no SSR.
        println!("SSR disabled (mode={}, vite_dev={})", cfg_data.get_ref().mode, vite_dev);
        None
    };

    // Run the server
    let res = server.run().await;

    // Ensure SSR node is terminated if it was started
    if let Some(node) = node {
        std::mem::drop(node.kill());
    }

    res
}
