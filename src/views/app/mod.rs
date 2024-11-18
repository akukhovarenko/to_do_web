mod content_loader;
mod items;
mod login;
mod logout;

use actix_web::web;
use std::path::Path;

pub fn app_factory(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(items::items));
    let base_path = Path::new("/api/v1");
    app.route(
            base_path.join("login").to_str().unwrap(),
            web::get().to(login::login),
        )
        .route(
            base_path.join("logout").to_str().unwrap(),
            web::get().to(logout::logout),
        );
}
