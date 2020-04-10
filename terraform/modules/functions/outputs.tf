output "service_name" {
    value = google_cloud_run_service.default.name
}

output "status" {
    value = google_cloud_run_service.default.status
}

output "container_image" {
    value = google_cloud_run_service.default.template[0].spec[0].containers[0].image
}
