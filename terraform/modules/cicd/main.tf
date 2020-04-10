# The following resource requires some manual setup on GitHub.
# See the README for more info.

resource "google_cloudbuild_trigger" "filename-trigger" {
    provider = google-beta
    description = "Push to any branch"
    filename = "cloudbuild.yaml"
    project = var.gcp_project

    github {
        owner = var.repo_owner
        name = var.repo_name
        push {
            branch = ".*"
        }
    }

    substitutions = {
        _CLOUD_RUN_REGION = var.region
        _CLOUD_RUN_SERVICE_NAME = var.service_name
    }
}
