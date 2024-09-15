use actix_web::{web, HttpResponse};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::state::load_state_from_file;
use super::utils::return_state;
use crate::process::process_input;
use crate::to_do::to_do_factory;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse{
    let state_file_name = "state.json";
    let state = load_state_from_file(&state_file_name);

    let status = match state.get(&to_do_item.title) {
        Some(status) => status.to_string().replace('\"', ""),
        None => return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title))
    };
    if &status == &to_do_item.status {
        return HttpResponse::Ok().json(return_state());
    }
    match to_do_factory(&status, &to_do_item.title) {
        Ok(item) => process_input(item, String::from("edit"), state, &state_file_name),
        Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status))
    }

    HttpResponse::Ok().json(return_state())
}