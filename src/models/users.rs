use chrono::NaiveDateTime;
use diesel::{associations::Identifiable, deserialize::Queryable};
use rocket_db_pools::diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
