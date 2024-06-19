use std::process::Command;

use common::APP_HOST;
use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::json;
use serde_json::Value;

pub mod common;

#[test]
fn test_login() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();
    println!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "usename": "test_admin",
            "password": "1234"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);

    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": "test_admin",
            "password": "12314"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
