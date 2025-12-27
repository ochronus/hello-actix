use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{App, HttpServer, web};

mod config;
mod handlers;

async fn manual_hello() -> &'static str {
    "Hey there!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load typed configuration (reads .env, files, and environment variables)
    let cfg: config::AppConfig =
        config::AppConfig::load().map_err(|e| std::io::Error::other(e.to_string()))?;
    let bind_port = cfg.effective_port();
    let cfg_data: web::Data<config::AppConfig> = web::Data::new(cfg);

    HttpServer::new({
        let cfg_data = cfg_data.clone();
        move || {
            let cfg = cfg_data.get_ref();
            App::new()
                .app_data(cfg_data.clone())
                .wrap(IdentityMiddleware::default())
                .wrap(
                    SessionMiddleware::builder(
                        CookieSessionStore::default(),
                        cfg.secret_key.clone_key(),
                    )
                    .cookie_name(cfg.cookie_name.clone())
                    .cookie_secure(cfg.cookie_secure())
                    .session_lifecycle(PersistentSession::default().session_ttl(cfg.cookie_ttl()))
                    .build(),
                )
                .configure(handlers::init)
                .route("/hey", web::get().to(manual_hello))
        }
    })
    .bind(("0.0.0.0", bind_port))?
    .run()
    .await
}
