#[macro_use]
extern crate diesel;
extern crate dotenv;
mod database;
mod json_serialization;
mod model;
mod schema;
mod to_do;
mod views;

use actix_web::{dev::Service, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                if req.path().contains("/item/") {
                    match views::token::process_token(&req) {
                        Ok(_token) => println!("Token ok"),
                        Err(message) => println!("Token error: {}", message),
                    }
                }
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
