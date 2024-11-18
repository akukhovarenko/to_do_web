use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};

use super::utils::return_state;
use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;
use crate::auth::jwt::JwtToken;


pub async fn edit(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let mut connection = establish_connection();
    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    let _ = diesel::update(
        to_do::table
            .filter(to_do::columns::title.eq(&to_do_item.title))
            .filter(to_do::columns::user_id.eq(&token.user_id))
        )
        .set(to_do::columns::status.eq("done"))
        .execute(&mut connection);

    HttpResponse::Ok().json(return_state(&token.user_id))
}
