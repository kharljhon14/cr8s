use chrono::NaiveDateTime;
use diesel::deserialize::Queryable;
use rocket_db_pools::diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
