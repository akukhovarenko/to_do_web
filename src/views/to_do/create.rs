use diesel::prelude::*;

use actix_web::{HttpRequest, HttpResponse};

use super::utils::return_state;
use crate::database::establish_connection;
use crate::models::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;
use crate::auth::jwt::JwtToken;


pub async fn create(req: HttpRequest) -> HttpResponse {
    let title = req.match_info().get("title").unwrap().to_string();
    let token:JwtToken = JwtToken::decode_from_request(req).unwrap();

    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    if items.is_empty() {
        let _ = diesel::insert_into(to_do::table)
            .values(NewItem::new(title, token.user_id.clone()))
            .execute(&mut connection);
    }

    HttpResponse::Ok().json(return_state(&token.user_id))
}
