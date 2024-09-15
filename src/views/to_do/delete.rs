use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::json_serialization::to_do_item::ToDoItem;
use crate::database::establish_connection;
use crate::model::item::item::Item;
use crate::schema::to_do;
use super::utils::return_state;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    for item in items {
        let _ = diesel::delete(&item)
            .execute(&mut connection);
    }

    HttpResponse::Ok().json(return_state())
}
