use rocket::{
    fairing::{Fairing, Info, Kind},
    Request, Response,
};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in response",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}
