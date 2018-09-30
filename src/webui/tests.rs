use super::super::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn index() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
}
