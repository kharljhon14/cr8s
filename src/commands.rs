use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{
    models::users::NewUser,
    respositories::{role_repository::RoleRepository, user_repository::UserRepository},
};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot retrived DB url from env");

    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Posgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection().await;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = NewUser {
        username,
        password: password_hash,
    };
    let user = UserRepository::create(&mut connection, new_user, role_codes)
        .await
        .unwrap();

    println!("User created {:?}", user);

    let roles = RoleRepository::find_by_user(&mut connection, &user)
        .await
        .unwrap();

    println!("Roles assigned {:?}", roles);
}

pub async fn list_users() {
    let mut connection = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut connection)
        .await
        .unwrap();

    if users.is_empty() {
        println!("No current users");
    } else {
        for user in users {
            println!("{:?}", user)
        }
    }
}

pub async fn delete_user(id: i32) {
    let mut connection = load_db_connection().await;
    UserRepository::delete(&mut connection, id).await.unwrap();

    println!("User with ID: {} is deleted", id);
}
