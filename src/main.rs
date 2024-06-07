use rocket_db_pools::Database;

use routes::rustaceans::get_rustaceans;

mod models;
mod respositories;
mod routes;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConnection(rocket_db_pools::diesel::PgPool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DbConnection::init())
        .launch()
        .await;
}
