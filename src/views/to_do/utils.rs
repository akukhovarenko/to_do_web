use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::load_state_from_file;
use crate::to_do::to_do_factory;


pub fn return_state() -> ToDoItems {
    let state_file_name = "state.json";
    let state = load_state_from_file(state_file_name);
    let mut buffer = Vec::new();

    for (key, value) in state {
        buffer.push(
            to_do_factory(
                value.as_str().unwrap(), 
                &key).unwrap()
        )
    }
    ToDoItems::new(buffer)
}