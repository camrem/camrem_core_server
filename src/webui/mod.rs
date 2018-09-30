use std::collections::HashMap;
use rocket_contrib::Template;

use super::api;
use rocket;

pub fn mount(r: rocket::Rocket) -> rocket::Rocket {
    r.mount("/", routes![index])
}

#[get("/")]
fn index() -> Template {
    let mut context : HashMap<String, String> = HashMap::new();
    context.insert(String::from("camrem_version"), String::from(api::VERSION.unwrap_or("unknown")));
    Template::render("index", &context)
}

#[cfg(test)] mod tests;
