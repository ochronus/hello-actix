use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, post, web};

/// POST /logout
///
/// Logs the user out by clearing the attached identity (if any).
#[post("/logout")]
pub async fn logout(
    user: Option<Identity>,
    _cfg: web::Data<crate::config::AppConfig>,
) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }
    HttpResponse::Ok()
}
