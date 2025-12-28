use actix_identity::Identity;
use actix_web::{HttpRequest, Responder, get};
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};
use serde_json::json;

/// GET /login
///
/// Renders the Inertia "Login" page and includes an `auth` prop reflecting the
/// logged-in state (via actix-identity). The frontend page shows login/logout
/// actions based on this state.
#[get("/login")]
pub async fn login_page(req: HttpRequest, user: Option<Identity>) -> impl Responder {
    let auth = match user.and_then(|u| u.id().ok()) {
        Some(id) => json!({ "user": { "id": id } }),
        None => json!({ "user": null }),
    };

    let props = hashmap![
        "auth" => InertiaProp::data(auth),
    ];

    Inertia::render_with_props(&req, "Login".into(), props).await
}
