#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::{content, NamedFile};

#[get("/uv")]
fn uv() -> &'static str {
    "TODO"
}

/// Ping.
#[get("/ping")]
fn ping() -> &'static str {
    "Pong!"
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
        .mount("/api/v1", routes![uv, ping])
}

fn main() {
    // start server
    rocket().launch();
}
