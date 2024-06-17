use rocket::serde::json::Json;

#[derive(serde::Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<Credentials>) {}
