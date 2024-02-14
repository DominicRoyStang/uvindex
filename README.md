![UV Index Logo](./docs/uvindex-logo-with-text.svg)

> A command-line application that displays uv index information in your area.

⚠ This project is no longer maintained.

![UV Index Demo](./docs/uvindex-demo.gif)

# About
UV Index is a simple command line application that allows users to quickly get the UV index in their area.
A common use case is to quickly determine if you will need to wear sunscreen on any given day.

## Features
- Automatic location detection based on the calling IP
- Custom sun protection recommendations based on the current uv index
- Three verbosity levels

## Background

I built `uvindex` mostly as a side-project to sharpen my skills with the following technologies: **Rust**, **Docker multi-stage builds**, **Terraform**, and **Kubernetes**. I also learned how to create nice scalable svg logos using Figma to create the logo you see at the top of this README, in the `docs` folder, and on [uvindex.xyz](https://uvindex.xyz).

Despite being mainly a learning project, I do use `uvindex` frequently to convince my friends and family to apply sunscreen. People are often surprised by uv index and its related recommended precautions on a day that doesn't _feel_ like a high-uv day.

# Installation

To install the CLI on x86 ubuntu/debian, you can just download the latest [release](https://github.com/DominicRoyStang/uvindex/releases) binary, and move it to your `~/.local/bin` folder.

On other platforms, you'll have to compile the binary yourself:
* Install Rust and Cargo
* Run `cd services/cli`
* Run `cargo build --release`
* Rename the file in `services/cli/target/release/uvindex-cli` to `uvindex`
* Move this `uvindex` file to the `~/.local/bin` folder

---

# Contributing

Pull requests and forks are welcome. For major changes, please open an issue first to discuss what you would like to change.

Instructions on how to set up the project for local development and on the cloud are listed below.

## Local Development

Set up the following environment variables
* `WEATHERBIT_API_KEY`: create a [weatherbit](https://www.weatherbit.io/) account and set up an API key.
* `OPENWEATHER_API_KEY`: create an [openweather](https://openweathermap.org/) account and set up an API key.

To run the code in containers (recommended)
* Install Docker
* Install Docker-compose
* Run `make build` from the root of this repository (you only need to rebuild when a dockerfile is changed)

To run the code natively
* Install Rust and Cargo
* Run `cd services/backend` followed by `rustup override set nightly`
* If things are still not working, run `rustup update && cargo update` from services/backend and services/cli
* set `UVINDEX_BACKEND_PROTOCOL` environment variable to `http`
* set `UVINDEX_BACKEND_HOST` environment variable to `localhost`
* set `UVINDEX_BACKEND_PORT` environment variable to `3000`

### Run the code
**In containers:**
```bash
make start
```

**Natively:** (from `services/backend` and `services/cli`)
```bash
cargo run -- <args>
```

### Set up on Google Cloud Platform
This project currently runs on Google Cloud Platform (GCP), and is hence set up to quickly get deployed on GCP using terraform.
Here are the (high-level) steps.

1. Create a GCP project.
2. Buy a domain.
3. Set up a service account as per the instructions [here](https://learn.hashicorp.com/terraform/gcp/build). Give it `owner` permissions.
4. Set the service account email (listed in the uvindex-account.json) as a domain owner on GCP.
5. Due to limitations with terraform and GCP, there is one manual step to perform on Github prior to running terraform:
    Enable the cloud build app from github (but don't add the build trigger manually on GCP)
6. Use terraform and specify all the variables listed in the `terraform/variables.tf` file to generate everything.
7. Add the following roles to the cloud build service account: `Service Account User`, `Cloud Run Admin`.

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
