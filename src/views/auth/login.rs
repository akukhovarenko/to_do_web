use actix_web::{HttpResponse, Responder};

pub async fn login() -> impl Responder {
    HttpResponse::Ok().body(String::from("Login success"))
}
