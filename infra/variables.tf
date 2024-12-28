data "aws_partition" "current" {}

data "aws_region" "current" {}

data "aws_caller_identity" "current" {}

variable "cloudflare_api_token" {
  description = "API token for Cloudflare"
  type        = string
  sensitive   = true
}

variable "my_ip" {
  type      = string
  sensitive = true
}