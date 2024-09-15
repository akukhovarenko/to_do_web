use crate::json_serialization::to_do_items::ToDoItems;
use crate::to_do::to_do_factory;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::model::item::item::Item;
use crate::schema::to_do;

pub fn return_state() -> ToDoItems {
    let mut connection = establish_connection();
    let items = to_do::table
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    let mut buffer = Vec::new();

    for item in items {
        buffer.push(to_do_factory(&item.status, &item.title).unwrap())
    }
    ToDoItems::new(buffer)
}
