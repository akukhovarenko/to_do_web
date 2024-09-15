use super::base::Base;


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
