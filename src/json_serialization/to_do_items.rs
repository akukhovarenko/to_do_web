use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use actix_web::{Responder, HttpResponse, http::header::ContentType, body::BoxBody};
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,

    pub pending_items_count: i8,
    pub done_items_count: i8,
}

impl ToDoItems {
    pub fn new(items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_items_buffer = Vec::new();
        let mut done_items_buffer = Vec::new();

        for item in items {
            match item {
                ItemTypes::Pending(packed) => pending_items_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_items_buffer.push(packed.super_struct),
            }
        }
        let done_items_count = done_items_buffer.len() as i8;
        let pending_items_count = pending_items_buffer.len() as i8;
        return ToDoItems {
            pending_items: pending_items_buffer,
            done_items: done_items_buffer,
            pending_items_count: pending_items_count,
            done_items_count: done_items_count,
        };
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[cfg(test)]
mod to_do_items_test {
    use super::ToDoItems;
    use crate::to_do::{
        structs::{done::Done, pending::Pending},
        ItemTypes,
    };
    #[test]
    fn new_test() {
        let items = ToDoItems::new(vec![
            ItemTypes::Pending(Pending::new("pending_title_1")),
            ItemTypes::Pending(Pending::new("pending_title_2")),
            ItemTypes::Done(Done::new("done_title_1")),
            ItemTypes::Done(Done::new("done_title_2")),
            ItemTypes::Done(Done::new("done_title_3")),
        ]);
        assert_eq!(items.pending_items.len() as i8, items.pending_items_count);
        assert_eq!(items.done_items.len() as i8, items.done_items_count);
        assert_eq!(items.pending_items_count, 2);
        assert_eq!(items.done_items_count, 3);
        for i in items.done_items {
            assert_eq!(i.status, "done");
        }
        for i in items.pending_items {
            assert_eq!(i.status, "pending");
        }
    }
}
