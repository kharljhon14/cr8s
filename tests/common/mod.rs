use std::process::Command;

use reqwest::{
    blocking::{Client, ClientBuilder},
    header, StatusCode,
};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

// Client common functions

pub fn get_logged_in_client(username: &str, role: &str) -> Client {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg("1234")
        .arg(role)
        .output()
        .unwrap();

    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": username,
            "password": "1234"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert!(json.get("token").is_some());

    let header_value = format!(
        "Bearer {}",
        json["token"].as_str().expect("Token should be a string")
    );

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).expect("Invalid Header value"),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn get_client_with_logged_in_viewer() -> Client {
    get_logged_in_client("test_viewer", "viewer")
}

pub fn get_client_with_logged_in_admin() -> Client {
    get_logged_in_client("test_admin", "admin")
}

// Rustaceans common functions

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Foo bar",
            "email": "foobar@gmail.com"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

// Crates common functions

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustaceans_id": rustacean["id"],
            "code": "Foo",
            "name": "Crate",
            "version": "0.1",
            "description": "Crate description"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
