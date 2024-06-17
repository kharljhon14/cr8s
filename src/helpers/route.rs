use std::error::Error;

use rocket::{http::Status, response::status::Custom, serde::json::json};

use serde_json::Value;

use rocket_db_pools::Database;

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
