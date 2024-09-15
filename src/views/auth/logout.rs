use actix_web::{HttpResponse, Responder};

pub async fn logout() -> impl Responder {
    HttpResponse::Ok().body(String::from("<h1>Logout success</h1>"))
}
