use std::{env, error::Error};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UvIndexResponse {
    lat: f32,
    lon: f32,
    date_iso: String,
    date: i32,
    value: f32
}

pub fn current_uv_index() -> Result<f32, Box<dyn Error>> {
    let api_key = env::var("OPENWEATHER_API_KEY").expect("Unable to read OPENWEATHER_API_KEY environment variable");

    let client = reqwest::blocking::Client::new();
    let response: UvIndexResponse = client.get("https://api.openweathermap.org/data/2.5/uvi")
        .query(&[("appid", api_key), ("lat", String::from("45.41117")), ("lon", String::from("-75.69812"))])
        .send()?
        .json()?;

    Ok(response.value)
}
