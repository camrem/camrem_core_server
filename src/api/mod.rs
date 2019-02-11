pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

use rocket;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use gphoto as gp;

pub fn mount(r: rocket::Rocket) -> rocket::Rocket {
    r.mount("/api/", routes![version, camera_sleep])
}

#[get("/version")]
pub fn version() -> Json<JsonValue> {
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

#[get("/camera/sleep")]
pub fn camera_sleep(state: State<super::camera::CameraHandler>) -> Json<JsonValue> {
    state.sleep_from_secs(5);
    Json(json!({"status": "success"}))
}


#[cfg(test)] mod tests;
