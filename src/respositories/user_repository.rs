use diesel::QueryResult;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::users::{NewUser, User},
    schema::users,
};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_user: NewUser,
    ) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(connection)
            .await
    }
}
