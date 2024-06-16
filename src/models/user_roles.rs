use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
};
use rocket_db_pools::diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::users_roles;

use super::{roles::Role, users::User};

#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}
