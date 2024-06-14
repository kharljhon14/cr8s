use chrono::NaiveDateTime;
use diesel::{associations::Identifiable, deserialize::Queryable};
use rocket_db_pools::diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::roles;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}
