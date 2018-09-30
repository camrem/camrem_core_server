#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate gphoto;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;


use rocket_contrib::Template;

mod api;
mod webui;
mod camera;

fn rocket() -> rocket::Rocket {
    let r = rocket::ignite()
        .attach(Template::fairing());
    let r = webui::mount(r);
    let r = api::mount(r);

    return r
}

fn main() {
    println!("camrem_core_server with api v{}", api::VERSION.unwrap_or("unknown"));
    rocket().launch();
}
