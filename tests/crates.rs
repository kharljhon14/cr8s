use common::APP_HOST;
use reqwest::StatusCode;
use rocket::form::validate::Contains;
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_crate() {
    let client = common::get_client_with_logged_in_admin();
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
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

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

#[test]
fn test_get_crates() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let client = common::get_client_with_logged_in_viewer();
    let response = client
        .get(format!("{}/crates", APP_HOST))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert!(json.as_array().contains(&a_crate));

    let client = common::get_client_with_logged_in_admin();
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let a_crate = common::create_test_crate(&client, &rustacean);

    let client = common::get_client_with_logged_in_viewer();

    let response = client
        .get(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(
        a_crate,
        json!(json!({
            "id": a_crate["id"],
            "rustaceans_id": rustacean["id"],
            "code": "Foo",
            "name": "Crate",
            "version": "0.1",
            "description": "Crate description",
            "created_at": a_crate["created_at"]
        }))
    );

    let client = common::get_client_with_logged_in_admin();
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustaceans_id": rustacean["id"],
            "code": "Foozz",
            "name": "Crate",
            "version": "0.1",
            "description": "Crate description"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "code": "Foozz",
            "name": "Crate",
            "version": "0.1",
            "description": "Crate description",
            "rustaceans_id": rustacean["id"],
            "created_at": a_crate["created_at"]
        })
    );

    // Test author switching
    let rustacean2 = common::create_test_rustacean(&client);

    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustaceans_id": rustacean2["id"],
            "code": "Foozz",
            "name": "Crate",
            "version": "0.1",
            "description": "Crate description"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "code": "Foozz",
            "name": "Crate",
            "version": "0.1",
            "description": "Crate description",
            "rustaceans_id": rustacean2["id"],
            "created_at": a_crate["created_at"]
        })
    );

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_delete_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    common::delete_test_rustacean(&client, rustacean);
}
