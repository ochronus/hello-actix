use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{
    App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Result,
    cookie::{Key, time::Duration},
    get,
    http::StatusCode,
    post, web,
};

const FIVE_MINUTES: Duration = Duration::minutes(5);
#[get("/")]
async fn index(user: Option<Identity>) -> Result<HttpResponse> {
    if let Some(user) = user {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(format!("Welcome! {}", user.id().unwrap())))
    } else {
        Ok(HttpResponse::build(StatusCode::OK).content_type("text/html; charset=utf-8").body(
            "Welcome Anonymous!<br/>
                <form method='POST' action='/login'>
                <button type='submit'>log in</button>
                </form>",
        ))
    }
}
#[post("/login")]
async fn login(request: HttpRequest) -> impl Responder {
    // Some kind of authentication should happen here
    // e.g. password-based, biometric, etc.
    // [...]

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }
    HttpResponse::Ok()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env if present
    let _ = dotenvy::dotenv();

    // Read port from PORT env var, default to 1337
    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(1337);

    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth-example".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(FIVE_MINUTES))
                    .build(),
            )
            .service(index)
            .service(login)
            .service(logout)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
