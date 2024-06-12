use std::error::Error;

use rocket::{http::Status, response::status::Custom, serde::json::json};
use serde_json::Value;

pub mod crates;
pub mod rustaceans;

pub fn server_error(error: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", error);
    Custom(Status::InternalServerError, json!("Error"))
}
