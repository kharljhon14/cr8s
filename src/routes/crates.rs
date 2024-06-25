use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

use crate::{
    helpers::route::{server_error, DbConnection, EditorUser},
    models::crates::{Crate, NewCrate},
    respositories::crate_repository::CratesRespository,
};

// Get mutliple crates endpoint

#[rocket::get("/crates")]
pub async fn get_crates(
    mut db_connection: Connection<DbConnection>,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    let limit = 100;
    CratesRespository::find_multiple(&mut db_connection, limit)
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(|error| server_error(Box::new(error)))
}

// Get crate endpoint

#[rocket::get("/crates/<id>")]
pub async fn get_crate(
    mut db_connection: Connection<DbConnection>,
    id: i32,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    CratesRespository::find(&mut db_connection, id)
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(|error| server_error(Box::new(error)))
}

// Create crate endpoint

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db_connection: Connection<DbConnection>,
    new_crate: Json<NewCrate>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    CratesRespository::create(&mut db_connection, new_crate.into_inner())
        .await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|error| server_error(Box::new(error)))
}

// Update crate endpoint

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(
    mut db_connection: Connection<DbConnection>,
    id: i32,
    a_crate: Json<Crate>,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    CratesRespository::update(&mut db_connection, id, a_crate.into_inner())
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(|error| server_error(Box::new(error)))
}

// Delete crate endpoint

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut db_connection: Connection<DbConnection>,
    id: i32,
    _user: EditorUser,
) -> Result<NoContent, Custom<Value>> {
    CratesRespository::delete(&mut db_connection, id)
        .await
        .map(|_| NoContent)
        .map_err(|error| server_error(Box::new(error)))
}
