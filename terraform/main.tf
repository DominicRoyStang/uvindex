provider "google" {
  version = "3.15.0"

  credentials = file("${var.project_name}-account.json")

  project = var.gcp_project_id
  region  = var.gcp_region
  zone = var.gcp_zone
}

provider "google-beta" {
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
    zone = var.gcp_zone
    repo_name = var.remote_repo_name
    repo_owner = var.remote_repo_owner

    cluster_name = module.cluster.name
}
}
