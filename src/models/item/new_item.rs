use crate::schema::to_do;

#[derive(Insertable)]
#[diesel(table_name=to_do)]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub user_id: i32,
}

impl NewItem {
    pub fn new(title: String, user_id: i32) -> NewItem {
        NewItem {
            title,
            status: String::from("pending"),
            user_id,
        }
    }
}
