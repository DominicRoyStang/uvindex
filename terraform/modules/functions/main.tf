resource "google_cloud_run_service" "default" {
    name = "cloudrun-${var.backend_service_name}"
    location = var.region
    autogenerate_revision_name = true

    metadata {
        namespace = var.project_id
    }

    template {
        spec {
            containers {
                image = "gcr.io/${var.project_id}/${var.backend_service_name}:latest"
                env {
                    name = "WEATHERBIT_API_KEY"
                    value = var.weatherbit_api_key
                }
                env {
                    name = "OPENWEATHER_API_KEY"
                    value = var.openweather_api_key
                }
                ports {
                    container_port = 3000
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
