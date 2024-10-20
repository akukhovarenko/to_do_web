use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(title: &str, status: &str) -> Base {
        Base {
            title: title.to_string(),
            status: status.to_string(),
        }
    }
}

#[cfg(test)]
mod base_test {
    use super::Base;
    #[test]
    fn new_test() {
        let title = String::from("any_title");
        let status = String::from("any_status");
        let actual = Base::new(&title, &status);
        assert_eq!(title, actual.title);
        assert_eq!(status, actual.status);
    }
}
