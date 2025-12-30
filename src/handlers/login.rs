use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, Responder, post};

/// POST /login
///
/// Performs a mock authentication and attaches an identity to the session,
/// then redirects to the home page where the frontend (Inertia) reflects the
/// authenticated state.
#[post("/login")]
/// [TLA+ Action] SessionModel!Login
/// Corresponds to the Login action in specs/SessionModel.tla
/// "Fresh ID" requirement is satisfied by CookieSessionStore's random UUID generation.
pub async fn login(request: HttpRequest) -> impl Responder {
    // In a real application, you'd verify credentials here.
    let _ = Identity::login(&request.extensions(), "User1".into());

    // Redirect back to the index route so the frontend can render auth state.
    actix_web::web::Redirect::to("/").see_other()
}
