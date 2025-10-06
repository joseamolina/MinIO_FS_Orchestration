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
  minio_server   = "127.0.0.1:9000"
  minio_user     = "jamolina"
  minio_password = "du7s!jd9sQ2"
}
