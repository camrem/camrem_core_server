pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

use rocket;
use rocket_contrib::{Json, Value};

pub fn mount(r: rocket::Rocket) -> rocket::Rocket {
    r.mount("/api/", routes![version])
}

#[get("/version")]
pub fn version() -> Json<Value> {
    Json(json!({ "version": VERSION.unwrap_or("unknown") }))
}


#[cfg(test)] mod tests;
