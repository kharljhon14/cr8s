use common::APP_HOST;
use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

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
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustaceans_id": rustacean["id"],
            "code": "Foo",
            "name": "Crate",
            "version": "0.1",
            "description": "Crate description",
            "created_at": a_crate["created_at"]
        })
    );

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}
