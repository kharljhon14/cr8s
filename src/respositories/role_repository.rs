use diesel::QueryResult;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::roles::{NewRole, Role},
    schema::roles,
};

pub struct RoleRepository;

impl RoleRepository {
    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_role: NewRole,
    ) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(connection)
            .await
    }
}
