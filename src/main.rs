#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod rest_api;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![rest_api::version])
}

fn main() {
    println!("camrem_core_server with api v{}", rest_api::VERSION.unwrap_or("unknown"));
    rocket().launch();
}
