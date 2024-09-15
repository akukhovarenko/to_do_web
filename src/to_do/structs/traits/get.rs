use super::item::Item;
use serde_json::value::Value;
use serde_json::Map;

pub trait Get
where
    Self: Item,
{
    fn get(&self, state: Map<String, Value>) {
        let item = state.get(self.get_title());
        match item {
            Some(result) => {
                println!("{} -> {}", self.get_title(), result);
            }
            None => println!("Item was not found: {}", self.get_title()),
        }
    }
}

#[cfg(test)]
mod get_test {
    use crate::to_do::structs::traits::get::Get;
    use crate::to_do::structs::traits::item::Item;
    use serde_json::{json, Map, Value};

    struct DoneTestStruct;

    impl Item for DoneTestStruct {
        fn get_status(&self) -> &str {
            "done"
        }
        fn get_title(&self) -> &str {
            "any done title"
        }
    }
    impl Get for DoneTestStruct {}

    #[test]
    fn get_test() {
        let actual = DoneTestStruct {};
        let mut state: Map<String, Value> = Map::new();
        state.insert(actual.get_title().to_string(), json!(actual.get_status()));
        actual.get(state)
    }

    #[test]
    fn get_test_not_in_state() {
        let actual = DoneTestStruct {};
        let mut state: Map<String, Value> = Map::new();
        state.insert(String::from("unknown title"), json!(actual.get_status()));
        actual.get(state)
    }
}
