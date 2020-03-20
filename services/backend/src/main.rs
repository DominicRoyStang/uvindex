#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/uv")]
fn uv() -> &'static str {
    "TODO"
}

#[get("/ping")]
fn ping() -> &'static str {
    "Pong!"
}

#[get("/")]
fn index() -> &'static str {
    "Backend server for uvindex.\n\
    More information at https://github.com/DominicRoyStang/uvindex/\n\n\
    Looking for the api? try https://uvindex.xyz/api/v1/ping"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/v1", routes![uv, ping])
}

fn main() {
    rocket().launch();
}
