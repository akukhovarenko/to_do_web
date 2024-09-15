pub trait Item: {
    fn get_title(&self) -> &str;
    fn get_status(&self) -> &str;
}