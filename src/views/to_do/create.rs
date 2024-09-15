use diesel::prelude::*;

use actix_web::{HttpRequest, HttpResponse};

use crate::database::establish_connection;
use crate::model::item::new_item::NewItem;
use crate::model::item::item::Item;
use crate::schema::to_do;
use super::utils::return_state;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let title = req.match_info().get("title").unwrap().to_string();
    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    if items.len() == 0 {
        let _ = diesel::insert_into(to_do::table).values(NewItem::new(title))
            .execute(&mut connection);
    }

    HttpResponse::Ok().json(return_state())
}
