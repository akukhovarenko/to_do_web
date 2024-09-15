use super::utils::return_state;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::process::process_input;
use crate::state::load_state_from_file;
use crate::to_do::to_do_factory;
use actix_web::{web, HttpResponse};

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state_file_name = "state.json";
    let state = load_state_from_file(&state_file_name);
    println!("!!!!");

    match to_do_factory(&to_do_item.status, &to_do_item.title) {
        Ok(item) => process_input(item, String::from("delete"), state, &state_file_name),
        Err(_item) => {
            return HttpResponse::BadRequest().json(format!("{} not accepted", to_do_item.status))
        }
    }

    HttpResponse::Ok().json(return_state())
}
