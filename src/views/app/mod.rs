mod content_loader;
mod items;
mod login;
mod logout;

use actix_web::web;
use std::path::Path;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path = Path::new("/");
    app.route(base_path.to_str().unwrap(), web::get().to(items::items))
        .route(
            base_path.join("login").to_str().unwrap(),
            web::get().to(login::login),
        )
        .route(
            base_path.join("logout").to_str().unwrap(),
            web::get().to(logout::logout),
        );
}
