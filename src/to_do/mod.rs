pub mod structs;

use structs::done::Done;
use structs::pending::Pending;

#[derive(Debug)]
pub enum ItemTypes {
    Done(Done),
    Pending(Pending),
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    match item_type {
        "done" => Ok(ItemTypes::Done(Done::new(item_title))),
        "pending" => Ok(ItemTypes::Pending(Pending::new(item_title))),
        _ => Err("Unknown item type"),
    }
}

#[cfg(test)]
mod to_do_test {
    use super::{to_do_factory, ItemTypes};

    #[test]
    fn to_do_factory_done() {
        let title = "any_title_done";
        let result = to_do_factory("done", title);
        match result {
            Ok(ItemTypes::Done(data)) => {
                assert_eq!(data.super_struct.title, title);
                assert_eq!(data.super_struct.status, "done");
            }
            _ => assert!(false, "ERROR: Incorrect return for done"),
        }
    }

    #[test]
    fn to_do_factory_pending() {
        let title = "any_title_pending";
        let result = to_do_factory("pending", title);
        match result {
            Ok(ItemTypes::Pending(data)) => {
                assert_eq!(data.super_struct.title, title);
                assert_eq!(data.super_struct.status, "pending");
            }
            _ => assert!(false, "ERROR: Incorrect return for pending"),
        }
    }

    #[test]
    fn to_do_factory_unknown() {
        let title = "any_title_unknown";
        let result = to_do_factory("unknown", title);
        match result {
            Err(message) => {
                assert_eq!(message, "Unknown item type");
            }
            _ => assert!(false, "ERROR: Incorrect return for unknown"),
        }
    }
}
