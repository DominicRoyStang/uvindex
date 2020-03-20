use std::{env, error::Error};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BackendResponse {
    uv_index: f32
}

pub fn current_uv_index(latitude: f32, longitude: f32) -> Result<f32, Box<dyn Error>> {
    let backend_url = env::var("BACKEND_URL").unwrap_or(String::from("https://uvindex.xyz"));

    /*
    let client = reqwest::blocking::Client::new();
    let response: BackendResponse = client.get(format!("{}/api/v1.0/current", backend_url))
        .query(&[("lat", latitude.to_string()), ("lon", longitude.to_string())])
        .send()?
        .json()?;

    Ok(response.uv_index)
    */
    Ok(4.20)
}
