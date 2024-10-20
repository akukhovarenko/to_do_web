use actix_web::web;

mod app;
mod auth;
mod to_do;
mod users;

pub fn views_factory(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(app::items::items));
    auth::auth_factory(app);
    to_do::item_factory(app);
    users::user_factory(app);
}
