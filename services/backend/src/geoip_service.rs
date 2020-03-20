use std::error::Error;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32
}

pub fn current_position() -> Result<Position, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response: Position = client.get("https://api.geoip.rs/").send()?.json()?;

    Ok(response)
}
