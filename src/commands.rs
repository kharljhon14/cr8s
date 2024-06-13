use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{models::users::NewUser, respositories::user_repository::UserRepository};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot retrived DB url from env");

    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Posgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection().await;

    let new_user = NewUser { username, password };
    UserRepository::create(&connection, new_user);
}

pub async fn list_users() {
    let connection = load_db_connection().await;
}

pub async fn delete_user(id: i32) {
    let connection = load_db_connection().await;
}
