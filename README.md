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
* Run `make build` from the root of this repository (you only need to rebuild when a dockerfile is changed)

To run the code natively (not recommended)
* Install Rust and Cargo
* Run `cd services/backend` followed by `rustup override set nightly`
* If things are still not working, run `rustup update && cargo update` from services/backend and services/cli
* set `UVINDEX_BACKEND_PROTOCOL` environment variable to `http`
* set `UVINDEX_BACKEND_HOST` environment variable to `localhost`
* set `UVINDEX_BACKEND_PORT` environment variable to `8080`

## Run the code (containers)
```bash
make start
```

## Run the code (natively)
```bash
cargo run -- <args>
```

## Kubernetes Local Setup
- Install kubectl
- Install minikube
- Follow the instructions in the comment at the top of the `kubernetes/secret_template.yaml` file
- Run these commands:
```bash
    eval $(minikube -p minikube docker-env) # To use local images
    docker build -t uvindex-backend services/backend/.
    kubectl apply -f kubernetes/.
    minikube service uvindex-load-balancer --url # Go to this url on a browser
```

## Set up on Google Cloud Platform
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
