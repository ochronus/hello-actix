/*! Handlers module.

This module organizes request handlers into separate submodules and re-exports
their route functions for convenient `.service(...)` registration in `main`.

Submodules:
- `index`  -> `GET /`
- `login`  -> `POST /login`
- `logout` -> `POST /logout`
- `echo`   -> `POST /echo`
*/

use actix_web::web;

pub mod echo;
pub mod index;
pub mod login;
pub mod logout;

/// Configure all handler services on the given ServiceConfig.
///
/// Usage:
///     app.configure(handlers::init);
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index).service(login::login).service(logout::logout).service(echo::echo);
}
