use super::item::Item;
use crate::state::save_state_to_file;
use serde_json::{Value, Map, json};


pub trait Create where Self: Item{
    fn create(&self, state: &mut Map<String, Value>, state_file_name: &str) {
        state.insert(self.get_title().to_string(), json!(self.get_status()));
        save_state_to_file(state_file_name, state);
    }
}


#[cfg(test)]
mod create_test {
    use serde_json::{json, Map, Value};
    use crate::state::load_state_from_file;
    use super::{Create, Item};
    use tempfile::tempdir;

    struct PendingTestStruct;
    impl Item for PendingTestStruct {
        fn get_status(&self) -> &str {
            "pending"
        }
        fn get_title(&self) -> &str {
            "any pending title"
        }
    }
    impl Create for PendingTestStruct {}

    #[test]
    fn create_test() {
        let dir = tempdir().unwrap();
        let buffer = dir
            .path()
            .join("create_test.json");
        let state_file_name = buffer
            .to_str()
            .unwrap();
        let actual = PendingTestStruct {};
        let mut state: Map<String, Value> = Map::new();
        state.insert(actual.get_title().to_string(), json!(actual.get_status()));
        actual.create(&mut state, state_file_name);
        let actual_state = load_state_from_file(state_file_name);
        assert_eq!(
            actual_state.get(actual.get_title()),
            Some(json!("pending")).as_ref()
        );
        dir.close().unwrap();
    }
}