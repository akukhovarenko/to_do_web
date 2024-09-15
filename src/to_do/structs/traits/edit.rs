use super::item::Item;
use crate::state::save_state_to_file;
use serde_json::{json, Map, Value};



pub trait Edit
where
    Self: Item,
{
    fn set_status(&self, state: &mut Map<String, Value>, status: &str, state_file_name: &str) {
        state.insert(self.get_title().to_string(), json!(status));
        save_state_to_file(state_file_name, state);
    }
    fn set_to_done(&self, state: &mut Map<String, Value>, state_file_name: &str) {
        self.set_status(state, "done", state_file_name);
    }
    fn set_to_pending(&self, state: &mut Map<String, Value>, state_file_name: &str) {
        self.set_status(state, "pending", state_file_name);
    }
}

#[cfg(test)]
mod edit_test {
    use crate::state::load_state_from_file;
    use super::{Edit, Item};
    use serde_json::{json, Map, Value};
    use tempfile::tempdir;

    struct DoneTestStruct;

    impl Item for DoneTestStruct {
        fn get_status(&self) -> &str {
            "done"
        }
        fn get_title(&self) -> &str {
            "any done title"
        }
    }
    impl Edit for DoneTestStruct {}
    struct PendingTestStruct;
    impl Item for PendingTestStruct {
        fn get_status(&self) -> &str {
            "pending"
        }
        fn get_title(&self) -> &str {
            "any pending title"
        }
    }
    impl Edit for PendingTestStruct {}

    #[test]
    fn set_done_test() {
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
        actual.set_to_done(&mut state, state_file_name);
        let actual_state = load_state_from_file(state_file_name);
        assert_eq!(
            actual_state.get(actual.get_title()),
            Some(json!("done")).as_ref()
        );
        dir.close().unwrap();
    }

    #[test]
    fn set_pending_test() {
        let dir = tempdir().unwrap();
        let buffer = dir
            .path()
            .join("create_test.json");
        let state_file_name = buffer
            .to_str()
            .unwrap();
        let actual = DoneTestStruct {};
        let mut state: Map<String, Value> = Map::new();
        state.insert(actual.get_title().to_string(), json!(actual.get_status()));
        actual.set_to_pending(&mut state, state_file_name);
        let actual_state = load_state_from_file(state_file_name);
        assert_eq!(
            actual_state.get(actual.get_title()),
            Some(json!("pending")).as_ref()
        );
        dir.close().unwrap();
    }
}
