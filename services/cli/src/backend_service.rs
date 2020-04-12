use std::{env, error::Error};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BackendResponse {
    uv_index: f32
}

pub fn current_uv_index(latitude: f32, longitude: f32) -> Result<f32, Box<dyn Error>> {
    let backend_protocol = env::var("UVINDEX_BACKEND_PROTOCOL").unwrap_or(String::from("http"));
    let backend_host = env::var("UVINDEX_BACKEND_HOST").unwrap_or(String::from("uvindex.xyz"));
    let backend_port = env::var("UVINDEX_BACKEND_PORT").unwrap_or(String::from("80"));
    let backend_url = format!("{}://{}:{}", backend_protocol, backend_host, backend_port);

    let client = reqwest::blocking::Client::new();
    let response: BackendResponse = client.get(&format!("{}/api/v1/current", backend_url))
        .query(&[("latitude", latitude.to_string()), ("longitude", longitude.to_string())])
        .send()?
        .json()?;

    Ok(response.uv_index)
}
