use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;

use super::utils::return_state;
use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::item::Item;
use crate::schema::to_do;
use crate::auth::jwt::JwtToken;


pub async fn delete(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let mut connection = establish_connection();
    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    for item in items {
        let _ = diesel::delete(&item).execute(&mut connection);
    }

    HttpResponse::Ok().json(return_state(&token.user_id))
}
