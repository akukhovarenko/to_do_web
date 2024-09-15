use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;
use super::utils::return_state;


pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse{
    let mut connection = establish_connection();
    let _ = diesel::update(to_do::table.filter(to_do::columns::title.eq(&to_do_item.title)))
        .set(to_do::columns::status.eq("done"))
        .execute(&mut connection);

    HttpResponse::Ok().json(return_state())
}