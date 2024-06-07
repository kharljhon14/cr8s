use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_db_pools::Connection;

use crate::{respositories::rustacean_respository::RustaceanRepository, DbConnection};

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(
    mut db_connection: Connection<DbConnection>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db_connection, 100)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
}
