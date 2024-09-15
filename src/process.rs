use serde_json::{Map, Value};

use crate::to_do::structs::done::Done;
use crate::to_do::structs::pending::Pending;
use crate::to_do::structs::traits::get::Get;
use crate::to_do::structs::traits::create::Create;
use crate::to_do::structs::traits::edit::Edit;
use crate::to_do::structs::traits::delete::Delete;
use crate::to_do::structs::traits::item::Item;
use crate::to_do::ItemTypes;



fn process_pending(item: Pending, command: String, state: Map<String, Value>, state_file_name: &str) {
    let mut state = state.clone();
    match command.as_str() {
       "get" => item.get(state),
       "create" => item.create(&mut state, state_file_name),
       "edit" => item.set_to_done(&mut state, state_file_name),
       "delete" => item.delete(&mut state, state_file_name),
       cmd => println!("Incorrect command '{}' for item '{}-{}", cmd, item.get_title(), item.get_status())
    }
}


fn process_done(item: Done, command: String, state: Map<String, Value>, state_file_name: &str) {
    let mut state = state.clone();
    match command.as_str() {
       "get" => item.get(state),
       "edit" => item.set_to_pending(&mut state, state_file_name),
       "delete" => item.delete(&mut state, state_file_name),
       cmd => println!("Incorrect command '{}' for item '{}-{}", cmd, item.get_title(), item.get_status())
    }
}


pub fn process_input(item: ItemTypes, command: String, state: Map<String, Value>, state_file_name: &str){
    match item {
        ItemTypes::Done(item) => process_done(item, command, state, state_file_name),
        ItemTypes::Pending(item) => process_pending(item, command, state, state_file_name)
    }
}

#[cfg(test)]
mod process_test {
    use serde_json::{Map, json};
    use crate::process::process_input;
    use crate::state::load_state_from_file;
    use crate::to_do::structs::{done::Done, pending::Pending};
    use crate::to_do::ItemTypes;
    use super::{process_done, process_pending};
    use tempfile::tempdir;

    #[test]
    fn process_pending_create_test(){
        let dir = tempdir().unwrap();
        let buffer = dir.path().join("testfile_pending.json");
        let state_file_name = buffer.to_str().unwrap();
        let title = "Title of pending";
        let item_pending = Pending::new(title);
        let state = Map::new();
        process_pending(item_pending, String::from("create"), state, state_file_name);
        let actual_state = load_state_from_file(state_file_name);
        assert_eq!(actual_state.get(title), Some(json!("pending")).as_ref());
        dir.close().unwrap();
    }
    #[test]
    fn process_done_test(){
        let dir = tempdir().unwrap();
        let buffer = dir.path().join("testfile_done.json");
        let state_file_name = buffer.to_str().unwrap();
        let title = "Title of pending";
        let item_done = Done::new(title);
        let state = Map::new();
        process_done(item_done, String::from("edit"), state, state_file_name);
        let actual_state = load_state_from_file(state_file_name);
        assert_eq!(actual_state.get(title), Some(json!("pending")).as_ref());
        dir.close().unwrap();
    }
    #[test]
    fn process_input_test(){
        let dir = tempdir().unwrap();
        let buffer = dir.path().join("testfile_input.json");
        let state_file_name = buffer.to_str().unwrap();
        let title = "Title of pending";
        let item_done = Done::new(title);
        let state = Map::new();
        process_input(ItemTypes::Done(item_done), String::from("edit"), state, state_file_name);
        let actual_state = load_state_from_file(state_file_name);
        assert_eq!(actual_state.get(title), Some(json!("pending")).as_ref());
        dir.close().unwrap();
    }

}