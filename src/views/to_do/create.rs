use super::utils::return_state;
use crate::process::process_input;
use crate::state::load_state_from_file;
use crate::to_do::to_do_factory;
use actix_web::{HttpRequest, HttpResponse};

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state_file_name = "state.json";
    let state = load_state_from_file(&state_file_name);
    let title = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory("pending", &title).expect("msg");
    process_input(item, "create".to_string(), state, state_file_name);
    HttpResponse::Ok().json(return_state())
}
