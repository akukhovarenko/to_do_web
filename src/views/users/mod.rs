mod create;
use actix_web::web;
use std::path::Path;

pub fn user_factory(app: &mut web::ServiceConfig) {
    let base_path = Path::new("/user");
    app.route(
        base_path.join("create").to_str().unwrap(),
        web::post().to(create::create),
    );
}
