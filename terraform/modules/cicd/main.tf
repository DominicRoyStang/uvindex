# The following resource requires some manual setup on GitHub.
# See the README for more info.

resource "google_cloudbuild_trigger" "filename-trigger" {
    provider = google-beta
    description = "Push to any branch"
    filename = "cloudbuild.yaml"
    project = var.gcp_project
    substitutions = {
        _CLOUDSDK_COMPUTE_ZONE = var.zone
        _CLOUDSDK_CONTAINER_CLUSTER = var.cluster_name
    }

    github {
        owner = var.repo_owner
        name = var.repo_name
        push {
            branch = ".*"
        }
    }
}
