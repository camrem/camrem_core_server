pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

use rocket_contrib::{Json, Value};

#[get("/version")]
pub fn version() -> Json<Value> {
    Json(json!({ "version": VERSION.unwrap_or("unknown") }))
}


#[cfg(test)]
mod test {
    use super::super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn version() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/version").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
