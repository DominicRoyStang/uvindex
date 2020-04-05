# The following resource needs to be created manually. See the README for more info.
#resource "google_sourcerepo_repository" "primary" {
#    name = "${var.repo_host}_${var.repo_owner}_${var.repo_name}"
#    project = var.gcp_project
#}

resource "google_cloudbuild_trigger" "primary" {
    #depends_on = [google_sourcerepo_repository.primary]
    description = "Push to any branch"
    filename = "cloudbuild.yaml"
    project = var.gcp_project

    trigger_template {
        branch_name = ".*"
        project_id = var.gcp_project
        repo_name = "${var.repo_host}_${var.repo_owner}_${var.repo_name}"
    }
}
