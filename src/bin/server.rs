extern crate cr8s;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                cr8s::routes::authorization::login,
                cr8s::routes::rustaceans::get_rustaceans,
                cr8s::routes::rustaceans::get_rustacean,
                cr8s::routes::rustaceans::create_rustacean,
                cr8s::routes::rustaceans::update_rustacean,
                cr8s::routes::rustaceans::delete_rustacean,
                cr8s::routes::crates::get_crates,
                cr8s::routes::crates::get_crate,
                cr8s::routes::crates::create_crate,
                cr8s::routes::crates::update_crate,
                cr8s::routes::crates::delete_crate
            ],
        )
        .attach(cr8s::routes::cors::Cors)
        .attach(cr8s::helpers::route::CacheConnection::init())
        .attach(cr8s::helpers::route::DbConnection::init())
        .launch()
        .await;
}
