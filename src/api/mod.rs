pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

use rocket_contrib::{Json, Value};

#[get("/version")]
pub fn version() -> Json<Value> {
    Json(json!({ "version": VERSION.unwrap_or("unknown") }))
}


#[cfg(test)] mod tests;
