use bcrypt::{hash, DEFAULT_COST};
use diesel::Insertable;
use uuid::Uuid;

use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> NewUser {
        NewUser {
            username,
            email,
            password: hash(password, DEFAULT_COST).unwrap(),
            unique_id: Uuid::new_v4().to_string(),
        }
    }
}