use std::str::FromStr;

use chrono::{Datelike, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use tera::{Context, Tera};

use crate::{
    helpers::auth::hash_password,
    mail::HtmlMailer,
    models::{roles::RoleCode, users::NewUser},
    respositories::{
        crate_repository::CratesRespository, role_repository::RoleRepository,
        user_repository::UserRepository,
    },
};

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").expect("Cannont load template engine")
}

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot retrived DB url from env");

    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Posgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection().await;

    let new_user = NewUser {
        username,
        password: hash_password(password).unwrap(),
    };

    let role_enums = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v.as_str()).unwrap())
        .collect();

    let user = UserRepository::create(&mut connection, new_user, role_enums)
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

pub async fn digest_send(email: String, hours_since: i32) {
    let mut connection = load_db_connection().await;
    let tera = load_template_engine();

    let crates = CratesRespository::find_since(&mut connection, hours_since)
        .await
        .unwrap();

    if crates.len() > 0 {
        println!("Sending digest for {} crates", crates.len());
        let year = Utc::now().year();
        let mut context = Context::new();

        context.insert("crates", &crates);
        context.insert("year", &year);

        let smtp_host = std::env::var("SMTP_HOST").expect("Cannot retrived SMTP host from env");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot retrived SMTP username from env");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot retrived SMTP password from env");

        let mailer = HtmlMailer {
            template_engine: tera,
            smtp_host,
            smtp_username,
            smtp_password,
        };

        mailer.send(email, "email/digest.html", context).unwrap();
    }
}
