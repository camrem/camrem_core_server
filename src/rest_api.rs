pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[get("/version")]
pub fn version() -> &'static str {
    VERSION.unwrap_or("unknown")
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
