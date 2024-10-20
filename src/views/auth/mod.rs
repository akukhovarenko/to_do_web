use actix_web::web;
use std::path::Path;
mod login;
mod logout;

pub fn auth_factory(app: &mut web::ServiceConfig) {
    let base_path = Path::new("/auth");
    app.route(
        base_path.join("login").to_str().unwrap(),
        web::get().to(login::login),
    )
    .route(
        base_path.join("logout").to_str().unwrap(),
        web::get().to(logout::logout),
    );
}
