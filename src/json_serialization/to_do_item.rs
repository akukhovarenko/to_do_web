use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String,
}

impl Responder for ToDoItem {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[cfg(test)]
mod to_do_items_test {

    
    #[test]
    fn new_test() {
        String::from("title");String::from("pending");
    }
}
