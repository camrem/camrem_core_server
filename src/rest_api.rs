pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[get("/version")]
pub fn version() -> &'static str {
    VERSION.unwrap_or("unknown")
}
