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
                    value = var.weatherbit_api_key
                }
                env {
                    name = "OPENWEATHER_API_KEY"
                    value = var.openweather_api_key
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
