/*! Handlers module.

This module organizes request handlers into separate submodules and re-exports
their route functions for convenient `.service(...)` registration in `main`.

Submodules:
- `index`   -> `GET /`
- `contact` -> `GET /contact`
- `login_page` -> `GET /login`
- `logout_page` -> `GET /logout`
- `login`   -> `POST /login`
- `logout`  -> `POST /logout`
- `echo`    -> `POST /echo`
*/

use actix_web::web;

pub mod contact;
pub mod echo;
pub mod index;
pub mod login;
pub mod login_page;
pub mod logout;
pub mod logout_page;

/// Configure all handler services on the given ServiceConfig.
///
/// Usage:
///     app.configure(handlers::init);
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index)
        .service(contact::contact)
        .service(login_page::login_page)
        .service(logout_page::logout_page)
        .service(login::login)
        .service(logout::logout)
        .service(echo::echo);
}
