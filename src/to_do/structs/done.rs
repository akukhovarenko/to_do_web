use super::base::Base;

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
