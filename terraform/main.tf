provider "google" {
  version = "3.5.0"

  credentials = file("${var.project_name}-account.json")

  project = var.gcp_project_id
  region  = var.gcp_region
  zone = var.gcp_zone
}

resource "google_app_engine_application" "app" {
  project = var.gcp_project_id
  location_id = var.gcp_location
}

resource "google_app_engine_domain_mapping" "domain_mapping" {
  domain_name = var.domain_name

  ssl_settings {
    ssl_management_type = "AUTOMATIC"
  }
}
