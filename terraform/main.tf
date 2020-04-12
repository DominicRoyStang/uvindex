provider "google" {
  version = "3.16.0"

  credentials = file("${var.project_name}-account.json")

  project = var.gcp_project_id
  region  = var.gcp_region
  zone = var.gcp_zone
}

provider "google-beta" {
    version = "3.16.0"

    credentials = file("${var.project_name}-account.json")

    project = var.gcp_project_id
    region  = var.gcp_region
    zone = var.gcp_zone
}

module "registry" {
    source = "./modules/registry"

    gcp_project = var.gcp_project_id
}

module "cicd" {
    source = "./modules/cicd"

    gcp_project = var.gcp_project_id
    repo_name = var.remote_repo_name
    repo_owner = var.remote_repo_owner
    region = var.gcp_region

    service_name = module.functions.service_name
}

module "functions" {
    source = "./modules/functions"

    backend_service_name = var.backend_service_name
    project_id = var.gcp_project_id
    region = var.gcp_region
    domain = var.domain_name
    weatherbit_api_key = var.weatherbit_api_key
    openweather_api_key = var.openweather_api_key
}
