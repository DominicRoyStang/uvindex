# GENERAL

variable "project_name" {
    default = "uvindex"
}

variable "domain_name" {
    default = "uvindex.xyz"
}

# GCP PROJECT

variable "gcp_project_id" {
    description = "unique identifier given to the gcp project - not to be confused with project number"
    default = "uvindex-dominicroystang"
}

variable "gcp_location" {
    description = "TO REMOVE - this concept isn't used in this setup"
    default = "us-central"
}

variable "gcp_region" {
    description = "Some resources are hosted in specific locations denoted by their region (collection of zones)."
    default = "us-central1"
}

variable "gcp_zone" {
    description = "A zone is an isolated location within a region. Each region has 3 or more zones labeled a,b,c..."
    default = "us-central-1-c"
}

# REPO

variable "remote_repo_owner" {
    default = "DominicRoyStang"
}

variable "remote_repo_name" {
    default = "uvindex"
}

# BACKEND SERVICE

variable "backend_service_name" {
    default = "uvindex-backend"
}

variable "weatherbit_api_key" {
    type = string
}

variable "openweather_api_key" {
    type = string
}
