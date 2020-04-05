#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::{content, NamedFile};
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
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").ok()
}

/// Homepage logo.
#[get("/uvindex-logo-with-text.svg")]
pub fn logo_with_text() -> Option<NamedFile> {
    NamedFile::open("static/uvindex-logo-with-text.svg").ok()
}

/// Basic homepage.
#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(
        r"
        <title>UV Index</title>
        <img src='uvindex-logo-with-text.svg' />
        <br /><br />
        <div>Backend server for uvindex.<div>
        <div>More information at <a href='https://github.com/DominicRoyStang/uvindex/'>https://github.com/DominicRoyStang/uvindex/</a></div>
        <br />
        <div>Looking for the api? try <a href='https://uvindex.xyz/api/v1/ping'>https://uvindex.xyz/api/v1/ping</a></div>"
    )
}

/// Set up server and routes
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, favicon, logo_with_text])
        .mount("/api/v1", routes![current, ping])
}

fn main() {
    // start server
    rocket().launch();
}
