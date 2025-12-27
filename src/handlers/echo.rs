use actix_web::{HttpResponse, Responder, post, web};

/// POST /echo
///
/// Echoes the request body back to the client.
#[post("/echo")]
pub async fn echo(req_body: String, _cfg: web::Data<crate::config::AppConfig>) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
