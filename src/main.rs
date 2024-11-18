#[macro_use]
extern crate diesel;
extern crate dotenv;
mod auth;
mod database;
mod json_serialization;
mod models;
mod schema;
mod to_do;
mod views;

use actix_web::{dev::Service, App, HttpResponse, HttpServer};
use futures::future::{ok, Either};
use log::info;
use env_logger;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                let request_url: String = String::from(*&req.uri().path().clone());
                let passed: bool;
                if req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => passed = true,
                        Err(_message) => passed = false,
                    }
                } else {
                    passed = true;
                }

                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => Either::Right(
                        ok(
                        req.into_response(HttpResponse::Unauthorized().finish())
                    )),
                };

                async move {
                    let result = end_result.await?;
                    let status = &result.status();
                    info!("{} -> {}", request_url, status);
                    Ok(result)
                }
            })
            .configure(views::views_factory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
