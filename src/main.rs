use rocket_db_pools::Database;

use routes::rustaceans::{
    create_rustacean, delete_rustacean, get_rustacean, get_rustaceans, update_rustacean,
};

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
        .mount(
            "/",
            rocket::routes![
                get_rustaceans,
                get_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .attach(DbConnection::init())
        .launch()
        .await;
}
