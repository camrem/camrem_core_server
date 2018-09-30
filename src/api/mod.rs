pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

use rocket;
use rocket_contrib::{Json, Value};
use gphoto as gp;

pub fn mount(r: rocket::Rocket) -> rocket::Rocket {
    r.mount("/api/", routes![version])
}

#[get("/version")]
pub fn version() -> Json<Value> {
    let version = gp::libgphoto2_version();

    Json(json!({ "version": VERSION.unwrap_or("unknown"),
                 "gphoto": {
                    "libgphoto": version.version(),
                    "camlibs": version.camlibs(),
                    "compiler": version.compiler(),
                    "ltdl":  version.ltdl(),
                    "exif": version.exif()
                  }
                }))
}


#[cfg(test)] mod tests;
