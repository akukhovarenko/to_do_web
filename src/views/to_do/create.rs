use diesel::prelude::*;

use actix_web::{HttpRequest, HttpResponse};

use super::utils::return_state;
use crate::database::establish_connection;
use crate::model::item::item::Item;
use crate::model::item::new_item::NewItem;
use crate::schema::to_do;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let title = req.match_info().get("title").unwrap().to_string();
    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    if items.is_empty() {
        let _ = diesel::insert_into(to_do::table)
            .values(NewItem::new(title, 1))
            .execute(&mut connection);
    }

    HttpResponse::Ok().json(return_state())
}
