use argon2::{PasswordHash, PasswordVerifier};
use rocket::{response::status::Custom, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::{json, Value};

use crate::{
    helpers::route_errors::{server_error, DbConnection},
    respositories::user_repository::UserRepository,
};

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db_connection: Connection<DbConnection>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db_connection, &credentials.username)
        .await
        .map(|user| {
            let argon2 = argon2::Argon2::default();
            let db_hash = PasswordHash::new(&user.password).unwrap();

            let result = argon2.verify_password(credentials.password.as_bytes(), &db_hash);

            if result.is_ok() {
                return json!("Success");
            }
            return json!("Unauthorized");
        })
        .map_err(|error| server_error(Box::new(error)))
}
