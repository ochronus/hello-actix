use actix_identity::Identity;
use actix_web::{Responder, post, web};

/// POST /logout
///
/// Logs the user out by clearing the attached identity (if any)
/// and redirects to the home page so the Inertia frontend can
/// re-render the authentication state.
#[post("/logout")]
pub async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }

    web::Redirect::to("/").see_other()
}
