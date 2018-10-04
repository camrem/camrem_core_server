#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

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
        .attach(Template::fairing())
        .manage(camera::CameraHandler::new());
    let r = webui::mount(r);
    let r = api::mount(r);

    return r
}

fn main() {
    pretty_env_logger::init();
    info!("camrem_core_server with api v{}", api::VERSION.unwrap_or("unknown"));

    rocket().launch();
}
