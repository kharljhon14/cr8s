use common::{create_test_rustacean, delete_test_rustacean, APP_HOST};
use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

mod common;

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans", APP_HOST))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert!(json.as_array().unwrap().contains(&rustacean));

    delete_test_rustacean(&client, rustacean)
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Foo bar",
            "email": "foobar@gmail.com"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Foo bar",
            "email": "foobar@gmail.com",
            "created_at": rustacean["created_at"],
        })
    );
    delete_test_rustacean(&client, rustacean)
}

#[test]
fn test_get_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Foo bar",
            "email": "foobar@gmail.com",
            "created_at": rustacean["created_at"],
        })
    );
    delete_test_rustacean(&client, rustacean)
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .put(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .json(&json!({
            "name": "Foo BAR",
            "email": "fooobar@mail.com"
        }))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Foo BAR",
            "email": "fooobar@mail.com",
            "created_at": rustacean["created_at"],
        })
    );
    delete_test_rustacean(&client, rustacean)
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
