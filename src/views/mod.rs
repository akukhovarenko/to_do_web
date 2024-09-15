use actix_web::web;
mod auth;
mod to_do;
mod app;
pub mod token;

pub fn views_factory(app: &mut web::ServiceConfig) {
    app.route(
        "/",
        web::get().to(app::items::items),
    );
    auth::auth_factory(app);
    to_do::item_factory(app);
}