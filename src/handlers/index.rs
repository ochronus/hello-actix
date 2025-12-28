use actix_identity::Identity;
use actix_web::{HttpRequest, Responder, get};
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};
use serde_json::json;

/// GET /
///
/// Renders the Inertia "Index" page and includes an `auth` prop reflecting the
/// logged-in state (via actix-identity).
#[get("/")]
pub async fn index(req: HttpRequest, user: Option<Identity>) -> impl Responder {
    // Build the auth state from the current identity (if any).
    let auth = match user.and_then(|u| u.id().ok()) {
        Some(id) => json!({ "user": { "id": id } }),
        None => json!({ "user": null }),
    };

    // Example props; `version` and `message` are used by the default Index page.
    let props = hashmap![
        "auth" => InertiaProp::data(auth),
        "version" => InertiaProp::data("1"),
        "message" => InertiaProp::data("Hello from Inertia + Actix!")
    ];

    Inertia::render_with_props(&req, "Index".into(), props).await
}
