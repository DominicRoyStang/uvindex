#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_include_static_resources;
use rocket::response::content;
use rocket_include_static_resources::StaticResponse;
mod openweather_service;
mod weatherbit_service;

/// Get current UV index at the provided latitude and longitude.
#[get("/current?<latitude>&<longitude>")]
fn current(latitude: f32, longitude: f32) -> content::Json<String>  {
    let uv_index = match weatherbit_service::current_uv_index(latitude, longitude) {
        Ok(index) => index,
        Err(_) => openweather_service::current_uv_index(latitude, longitude).unwrap()
    };

    let body = format!("{{\"uv_index\": {}}}", uv_index);

    content::Json(body)
}

/// Ping.
#[get("/ping")]
fn ping() -> &'static str {
    "Pong!\n"
}

/// Main favicon.
#[get("/favicon.ico")]
pub fn favicon() -> StaticResponse {
    static_response!("favicon")
}

/// Logo.
#[get("/uvindex-logo.svg")]
pub fn logo() -> StaticResponse {
    static_response!("uvindex-logo")
}

/// Homepage logo.
#[get("/uvindex-logo-with-text.svg")]
pub fn logo_with_text() -> StaticResponse {
    static_response!("uvindex-logo-with-text")
}

/// Basic homepage.
#[get("/")]
fn index() -> StaticResponse {
    static_response!("index")
}

/// Set up server and routes
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach( // Initialize static resources to be included in release binary
            StaticResponse::fairing(|resources| {
                static_resources_initialize!(
                    resources,
                    "favicon", "static/favicon.ico",
                    "uvindex-logo", "static/uvindex-logo.svg",
                    "uvindex-logo-with-text", "static/uvindex-logo-with-text.svg",
                    "index", "static/index.html",
                );
            })
        )
        .mount("/", routes![index, favicon])
        .mount("/static", routes![favicon, logo, logo_with_text])
        .mount("/api/v1", routes![current, ping])
}

fn main() {
    // start server
    rocket().launch();
}
