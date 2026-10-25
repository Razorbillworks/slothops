terraform {
  required_version = ">= 1.5.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

variable "project_id" {
  type        = string
  description = "GCP Project ID to audit"
  default     = "slothops-gcp-audit"
}

provider "google" {
  project = var.project_id
  region  = "us-central1"
}

resource "google_service_account" "sloth_auditor" {
  account_id   = "slothops-auditor"
  display_name = "SlothOps FinOps Read-Only Auditor"
}

resource "google_project_iam_member" "viewer_role" {
  project = var.project_id
  role    = "roles/compute.viewer"
  member  = "serviceAccount:${google_service_account.sloth_auditor.email}"
}