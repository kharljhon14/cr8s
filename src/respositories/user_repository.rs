use diesel::{ExpressionMethods, GroupedBy, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{
        roles::{NewRole, Role},
        user_roles::{NewUserRole, UserRole},
        users::{NewUser, User},
    },
    schema::{roles, users, users_roles},
};

use super::role_repository::RoleRepository;

pub struct UserRepository;

impl UserRepository {
    pub async fn find(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(connection).await
    }

    pub async fn find_by_username(
        connection: &mut AsyncPgConnection,
        username: &String,
    ) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result(connection)
            .await
    }

    pub async fn find_with_roles(
        connection: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(connection).await?;
        let results = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(connection)
            .await?
            .grouped_by(&users);

        Ok(users.into_iter().zip(results).collect())
    }

    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_user: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(connection)
            .await?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) =
                    RoleRepository::find_by_code(connection, role_code.to_owned()).await
                {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        code: role_code.to_owned(),
                        name: role_code.to_owned().to_uppercase(),
                    };
                    let role = RoleRepository::create(connection, new_role).await?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(connection)
                .await?;
        }

        Ok(user)
    }

    pub async fn delete(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id)))
            .execute(connection)
            .await?;

        diesel::delete(users::table.find(id))
            .execute(connection)
            .await
    }
}
