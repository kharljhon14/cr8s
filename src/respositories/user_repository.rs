use diesel::QueryResult;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{
        roles::NewRole,
        user_roles::{NewUserRole, UserRole},
        users::{NewUser, User},
    },
    schema::{users, users_roles},
};

use super::role_repository::RoleRepository;

pub struct UserRepository;

impl UserRepository {
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
}
