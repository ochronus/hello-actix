use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, post, web};

/// POST /login
///
/// Attaches a verified user identity to the active session.
/// In a real application, you'd perform authentication here (passwords, etc.).
#[post("/login")]
pub async fn login(
    request: HttpRequest,
    _cfg: web::Data<crate::config::AppConfig>,
) -> impl Responder {
    // Some kind of authentication should happen here
    // e.g. password-based, biometric, etc.
    // [...]

    // Attach a verified user identity to the active session
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}
