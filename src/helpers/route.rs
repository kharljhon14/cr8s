use std::error::Error;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::status::Custom,
    serde::json::json,
    Request,
};

use serde_json::Value;

use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection, Database};

use crate::{
    models::{roles::RoleCode, users::User},
    respositories::{role_repository::RoleRepository, user_repository::UserRepository},
};

#[derive(Database)]
#[database("postgres")]
pub struct DbConnection(rocket_db_pools::diesel::PgPool);

#[derive(Database)]
#[database("redis")]
pub struct CacheConnection(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(error: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", error);
    Custom(Status::InternalServerError, json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_header = req
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let mut cache = req
                .guard::<Connection<CacheConnection>>()
                .await
                .expect("Can not connect to Redis in request guard");

            let mut db_connection = req
                .guard::<Connection<DbConnection>>()
                .await
                .expect("Can not connect to  PG database in request guard");

            let result = cache
                .get::<String, i32>(format!("sessions/{}", header_value[1]))
                .await;

            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db_connection, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

#[allow(dead_code)]
pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = req
            .guard::<User>()
            .await
            .expect("Cannot retrieve current logged in user");
        rocket::info!("User info {:?}", user);

        let mut db_connection = req
            .guard::<Connection<DbConnection>>()
            .await
            .expect("Cannot connect to Postgres in request guard");

        if let Ok(roles) = RoleRepository::find_by_user(&mut db_connection, &user).await {
            rocket::info!("Roles assigned are {:?}", roles);

            let is_editor = roles.iter().any(|role| match role.code {
                RoleCode::Admin => true,
                RoleCode::Editor => true,
                _ => false,
            });

            rocket::info!("Is Editor is {:?}", is_editor);

            if is_editor {
                return Outcome::Success(EditorUser(user));
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
