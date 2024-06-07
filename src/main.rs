use respositories::rustacean_respository::RustaceanRepository;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_db_pools::{Connection, Database};

mod models;
mod respositories;
mod routes;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConnection(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
async fn get_rustaceans(
    mut db_connection: Connection<DbConnection>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::get_multiple(&mut db_connection, 100)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DbConnection::init())
        .launch()
        .await;
}
