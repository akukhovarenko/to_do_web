use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;

pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
    let mut connection = establish_connection();
    let insert_result = diesel::insert_into(users::table)
        .values(NewUser::new(
            new_user.name.clone(),
            new_user.email.clone(),
            new_user.password.clone(),
        ))
        .execute(&mut connection);

    match insert_result {
        Ok(_) => HttpResponse::Created().await.unwrap(),
        Err(_) => HttpResponse::Conflict().await.unwrap(),
    }
}
