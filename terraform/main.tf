provider "google" {
  version = "3.15.0"

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
    repo_name = var.project_name
    repo_host = var.remote_repo_host
    repo_owner = var.remote_repo_owner
}
