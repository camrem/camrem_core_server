#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;


use rocket_contrib::Template;

mod api;
mod webui;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![webui::index])
        .mount("/api/", routes![api::version])
}

fn main() {
    println!("camrem_core_server with api v{}", api::VERSION.unwrap_or("unknown"));
    rocket().launch();
}
