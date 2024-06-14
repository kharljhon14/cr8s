use diesel::{query_dsl::methods::FilterDsl, BelongingToDsl, ExpressionMethods, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{
        roles::{NewRole, Role},
        user_roles::UserRole,
        users::User,
    },
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

    pub async fn find_by_ids(
        connection: &mut AsyncPgConnection,
        ids: Vec<i32>,
    ) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .load(connection)
            .await
    }

    pub async fn find_by_user(
        connection: &mut AsyncPgConnection,
        user: &User,
    ) -> QueryResult<Vec<Role>> {
        let user_roles: Vec<UserRole> = UserRole::belonging_to(&user)
            .get_results(connection)
            .await?;

        let role_ids: Vec<i32> = user_roles
            .iter()
            .map(|user_role: &UserRole| user_role.id)
            .collect();

        Self::find_by_ids(connection, role_ids).await
    }

    pub async fn find_by_code(
        connection: &mut AsyncPgConnection,
        code: String,
    ) -> QueryResult<Role> {
        roles::table
            .filter(roles::code.eq(code))
            .first(connection)
            .await
    }
}
