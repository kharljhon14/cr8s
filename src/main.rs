use rocket_db_pools::{Connection, Database};

mod models;
mod respositories;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConnection(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
fn get_rustaceans(db: Connection<DbConnection>) {}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DbConnection::init())
        .launch()
        .await;
}
