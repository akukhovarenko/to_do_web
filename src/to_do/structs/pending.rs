use super::base::Base;
use super::traits::create::Create;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;
use super::traits::item::Item;

#[derive(Debug)]
pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(title: &str) -> Pending {
        Pending {
            super_struct: Base::new(title, "pending"),
        }
    }
}

impl Item for Pending {
    fn get_title(&self) -> &str {
        &self.super_struct.title
    }
    fn get_status(&self) -> &str {
        &self.super_struct.status
    }
}
impl Create for Pending {}
impl Get for Pending {}
impl Edit for Pending {}
impl Delete for Pending {}

#[cfg(test)]
mod pending_test {
    use super::Pending;

    #[test]
    fn new() {
        let title = "any_title_pending";
        let actual = Pending::new(title);
        assert_eq!(title, actual.super_struct.title);
        assert_eq!("pending", actual.super_struct.status);
    }

}
