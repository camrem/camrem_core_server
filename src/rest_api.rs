pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[get("/version")]
pub fn version() -> &'static str {
    VERSION.unwrap_or("unknown")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
