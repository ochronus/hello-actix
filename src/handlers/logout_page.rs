use actix_identity::Identity;
use actix_web::{HttpRequest, Responder, get};
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};
use serde_json::json;

/// GET /logout
///
/// Renders an Inertia page ("Logout") that triggers a client-side POST to `/logout`.
/// This mirrors a common pattern where the GET route shows a transitional page
/// and the frontend performs the actual logout request.
///
/// The page receives the current `auth` state so it can show contextual UI while
/// the client-side logout happens.
#[get("/logout")]
pub async fn logout_page(req: HttpRequest, user: Option<Identity>) -> impl Responder {
    let auth = match user.and_then(|u| u.id().ok()) {
        Some(id) => json!({ "user": { "id": id } }),
        None => json!({ "user": null }),
    };

    let props = hashmap![
        "auth" => InertiaProp::data(auth),
    ];

    Inertia::render_with_props(&req, "Logout".into(), props).await
}
