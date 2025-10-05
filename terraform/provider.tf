terraform {
  required_providers {
    minio = {
      source  = "aminueza/minio"
      version = "3.5.2"
    }
  }
}

provider minio {
  // required
  minio_server   = "..."
  minio_user     = "..."
  minio_password = "..."

  // optional
  minio_region      = "..."
  minio_api_version = "..."
  minio_ssl         = "..."
}