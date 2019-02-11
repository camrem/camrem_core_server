#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate gphoto;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;

use rocket_contrib::templates::Template;

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
