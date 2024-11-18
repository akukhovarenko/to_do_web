use actix_web::web;
use std::path::Path;
mod create;
mod delete;
mod edit;
mod get;
mod utils;

pub fn item_factory(app: &mut web::ServiceConfig) {
    let base_path = Path::new("/api/v1/item");
    app.route(
        base_path.join("get").to_str().unwrap(),
        web::get().to(get::get),
    )
    .route(
        base_path.join("create/{title}").to_str().unwrap(),
        web::post().to(create::create),
    )
    .route(
        base_path.join("delete").to_str().unwrap(),
        web::post().to(delete::delete),
    )
    .route(
        base_path.join("edit").to_str().unwrap(),
        web::put().to(edit::edit),
    );
}
