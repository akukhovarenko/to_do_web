pub mod new_item;

use crate::schema::to_do;

#[derive(Queryable, Identifiable)]
#[diesel(table_name=to_do)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub user_id: i32,
}
