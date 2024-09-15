use super::base::Base;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;
use super::traits::item::Item;

#[derive(Debug)]
pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(title: &str) -> Done {
        Done {
            super_struct: Base::new(title, "done"),
        }
    }
}

impl Item for Done {
    fn get_title(&self) -> &str {
        &self.super_struct.title
    }
    fn get_status(&self) -> &str {
        &self.super_struct.status
    }
}

impl Get for Done {}

impl Edit for Done {}

impl Delete for Done {}

#[cfg(test)]
mod done_test {
    use super::Done;
    #[test]
    fn new() {
        let title = "any_title_done";
        let actual = Done::new(title);
        assert_eq!(title, actual.super_struct.title);
        assert_eq!("done", actual.super_struct.status);
    }
}
