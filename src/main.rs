use actix_session::{
    Session, SessionMiddleware, config::PersistentSession, storage::CookieSessionStore,
};

use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    cookie::{Key, time::Duration},
    get, post, web,
};

const FIVE_MINUTES: Duration = Duration::minutes(5);

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth-example".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(FIVE_MINUTES))
                    .build(),
            )
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 1337))?
    .run()
    .await
}
