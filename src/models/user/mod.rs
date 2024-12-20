pub mod new_user;

use bcrypt::verify;
use diesel::{Identifiable, Queryable};

use crate::schema::users;

#[derive(Queryable, Identifiable, Clone)]
#[diesel(table_name=users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl User {
    pub fn verify(self, password: String) -> bool {
        verify(password, &self.password).unwrap()
    }
}
