use std::error::Error;

use rocket::{http::Status, response::status::Custom, serde::json::json};

use serde_json::Value;

use rocket_db_pools::Database;

pub mod crates;
pub mod rustaceans;

#[derive(Database)]
#[database("postgres")]
pub struct DbConnection(rocket_db_pools::diesel::PgPool);

pub fn server_error(error: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", error);
    Custom(Status::InternalServerError, json!("Error"))
}
