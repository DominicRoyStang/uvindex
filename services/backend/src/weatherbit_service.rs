use std::{env, error::Error};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CurrentApiResponse {
    data: Vec<UvData>
}

#[derive(Debug, Serialize, Deserialize)]
struct UvData {
    lat: f32,
    lon: f32,
    city_name: String,
    state_code: String,
    country_code: String,
    uv: f32
}

pub fn current_uv_index(latitude: f32, longitude: f32) -> Result<f32, Box<dyn Error>> {
    let api_key = env::var("WEATHERBIT_API_KEY").expect("Unable to read WEATHERBIT_API_KEY environment variable");

    let client = reqwest::blocking::Client::new();
    let response: CurrentApiResponse = client.get("https://api.weatherbit.io/v2.0/current")
        .query(&[("key", api_key), ("lat", latitude.to_string()), ("lon", longitude.to_string())])
        .send()?
        .json()?;

    let data = &response.data[0];

    Ok(data.uv)
}
