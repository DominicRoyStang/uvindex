resource "google_cloud_run_service" "default" {
    name = "cloudrun-${var.project_name}-backend"
    location = var.region

    metadata {
        namespace = var.project_id
    }

    template {
        spec {
            containers {
                image = "gcr.io/${var.project_id}/${var.project_name}-backend:latest"
                env {
                    name = "WEATHERBIT_API_KEY"
                    # value needs to be set manually since it is a secret
                }
                env {
                    name = "OPENWEATHER_API_KEY"
                    # value needs to be set manually since it is a secret
                }
            }
        }
    }
}

resource "google_cloud_run_domain_mapping" "default" {
    location = var.region
    name = var.domain

    metadata {
        namespace = var.project_id
    }

    spec {
        route_name = google_cloud_run_service.default.name
    }
}
