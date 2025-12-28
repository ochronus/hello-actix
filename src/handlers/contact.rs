use actix_web::{HttpRequest, Responder, get};
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};
use serde_json::json;

/// GET /contact
///
/// Renders the Inertia "Contact" page with example user details,
/// mirroring the actix_ssr example.
#[get("/contact")]
pub async fn contact(req: HttpRequest) -> impl Responder {
    let props = hashmap![
        "user" => InertiaProp::always(json!({
            "name": "John Doe",
            "email": "johndoe@example.com"
        }))
    ];

    Inertia::render_with_props(&req, "Contact".into(), props).await
}
