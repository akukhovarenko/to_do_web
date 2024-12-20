use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::json_serialization::login::Login;
use crate::models::user::User;
use crate::schema::users;

pub async fn login(credentials: web::Json<Login>) -> impl Responder {
    let username = credentials.username.clone();
    let password = credentials.password.clone();
    println!("User login {}", username);
 
    let mut connection = establish_connection();
    let users = users::table
        .filter(users::columns::username.eq(username.as_str()))
        .load::<User>(&mut connection)
        .unwrap();
    if users.is_empty() {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }
    match users[0].clone().verify(password) {
        true => {
            let token = JwtToken::encode(users[0].clone().id);
            HttpResponse::Ok().append_header(("token", token)).await.unwrap()
        }
        false => {
            HttpResponse::Unauthorized().await.unwrap()
        }
    }
}
