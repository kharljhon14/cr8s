use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

use crate::{
    helpers::route::{server_error, DbConnection},
    models::rustaceans::{NewRustacean, Rustacean},
    models::users::User,
    respositories::rustacean_repository::RustaceanRepository,
};

// Get multiple rustaceans endpoint

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(
    mut db_connection: Connection<DbConnection>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    let limit = 100;

    RustaceanRepository::find_multiple(&mut db_connection, limit)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|error| server_error(Box::new(error)))
}

// Get rustacean endpoint

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(
    mut db_connection: Connection<DbConnection>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db_connection, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|error| server_error(Box::new(error)))
}

// Create rustacean endpoint

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db_connection: Connection<DbConnection>,
    new_rustacean: Json<NewRustacean>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db_connection, new_rustacean.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|error| server_error(Box::new(error)))
}

// Update rustacean endpoint

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    mut db_connection: Connection<DbConnection>,
    id: i32,
    rustacean: Json<Rustacean>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db_connection, id, rustacean.into_inner())
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|error| server_error(Box::new(error)))
}

// Delete rustacean endpoint

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db_connection: Connection<DbConnection>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db_connection, id)
        .await
        .map(|_| NoContent)
        .map_err(|error| server_error(Box::new(error)))
}
