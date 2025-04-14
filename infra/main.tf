terraform {
  backend "s3" {
    bucket         = "seafoodfry-tf-backend"
    key            = "kamkekai-infra"
    region         = "us-east-2"
    dynamodb_table = "terraform-state-lock"
    encrypt        = true
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.76"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 5.3"
    }
  }

  required_version = "~> 1.9.8"
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

provider "aws" {
  #alias  = "us_east_1"
  region = "us-east-1"
}

provider "aws" {
  alias  = "us_east_2"
  region = "us-east-2"
}

# resource "aws_dynamodb_table" "terraform_lock" {
#   name           = "terraform-state-lock"
#   billing_mode   = "PAY_PER_REQUEST"
#   hash_key       = "LockID"

#   attribute {
#     name = "LockID"
#     type = "S"
#   }
# }