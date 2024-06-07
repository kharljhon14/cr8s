use reqwest::{blocking::Client, StatusCode};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap_or_else(|err| panic!("Request failed {:?}", err));

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_create_rustacean() {}
