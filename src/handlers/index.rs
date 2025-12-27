use actix_identity::Identity;
use actix_web::{HttpResponse, Result, get, http::StatusCode, web};

#[get("/")]
pub async fn index(
    user: Option<Identity>,
    _cfg: web::Data<crate::config::AppConfig>,
) -> Result<HttpResponse> {
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
