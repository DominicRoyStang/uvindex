![UV Index Logo](./docs/uvindex-logo-with-text.svg)


# About
UV Index is a simple command line application that allows users to quickly get the UV index in their area.

The most common use case is to quickly determine if you will need to wear sunscreen on any given day.

# Environment Setup

Set up the following environment variables
* `WEATHERBIT_API_KEY`: create a [weatherbit](https://www.weatherbit.io/) account and set up an API key.
* `OPENWEATHER_API_KEY`: create an [openweather](https://openweathermap.org/) account and set up an API key.

To run the code in containers (recommended)
* Install Docker
* Install Docker-compose

To run the code natively (not recommended)
* Install Rust and Cargo
* Run `cd services/backend` followed by `rustup override set nightly`
* If things are still not working, run `rustup update && cargo update` from services/backend and services/cli
* set `UVINDEX_BACKEND_PROTOCOL` environment variable to `http`
* set `UVINDEX_BACKEND_HOST` environment variable to `localhost`
* set `UVINDEX_BACKEND_PORT` environment variable to `8000`

# Run the code (containers)
```bash
docker-compose up --build
```

# Run the code (natively)
```bash
cargo run -- <args>
```
