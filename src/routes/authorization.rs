use rocket::{http::Status, response::status::Custom, serde::json::Json};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use serde_json::{json, Value};

use crate::{
    helpers::{
        auth::{authorize_user, Credentials},
        route::{server_error, CacheConnection, DbConnection},
    },
    models::users::User,
    respositories::user_repository::UserRepository,
};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db_connection: Connection<DbConnection>,
    mut cache_connection: Connection<CacheConnection>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_username(&mut db_connection, &credentials.username)
        .await
        .map_err(|error| server_error(Box::new(error)))?;

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Invalid credentials")))?;

    let three_hours = 3 * 60 * 60;

    cache_connection
        .set_ex::<String, i32, ()>(format!("sessions/{}", session_id), user.id, three_hours)
        .await
        .map_err(|error| server_error(error.into()))?;

    Ok(json!({"token":session_id}))
}

#[rocket::get("/me")]
pub fn me(user: User) -> Value {
    json!(user)
}
