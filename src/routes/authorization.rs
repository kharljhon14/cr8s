use rocket::{response::status::Custom, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::{json, Value};

use crate::{
    helpers::{
        auth::{authorize_user, Credentials},
        route::{server_error, DbConnection},
    },
    respositories::user_repository::UserRepository,
};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db_connection: Connection<DbConnection>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db_connection, &credentials.username)
        .await
        .map(|user| {
            if let Ok(token) = authorize_user(&user, credentials.into_inner()) {
                return json!(token);
            }
            return json!("Unauthorized");
        })
        .map_err(|error| server_error(Box::new(error)))
}
