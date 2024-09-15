use super::item::Item;
use crate::state::save_state_to_file;
use serde_json::{Map, Value};

pub trait Delete
where
    Self: Item,
{
    fn delete(&self, state: &mut Map<String, Value>, state_file_name: &str) {
        state.remove(self.get_title());
        save_state_to_file(state_file_name, state);
    }
}

#[cfg(test)]
mod delete_test {
    use super::{Delete, Item};
    use crate::state::load_state_from_file;
    use serde_json::{json, Map, Value};
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
    impl Delete for PendingTestStruct {}

    #[test]
    fn delete_test() {
        let dir = tempdir().unwrap();
        let buffer = dir.path().join("create_test.json");
        let state_file_name = buffer.to_str().unwrap();
        let actual = PendingTestStruct {};
        let mut state: Map<String, Value> = Map::new();
        state.insert(actual.get_title().to_string(), json!(actual.get_status()));

        actual.delete(&mut state, state_file_name);
        let actual_state = load_state_from_file(state_file_name);
        assert_eq!(actual_state.get(actual.get_title()), None);
        dir.close().unwrap();
    }
}
