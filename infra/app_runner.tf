resource "aws_apprunner_service" "kamekai" {
  service_name = "kamekai-backend"

  source_configuration {
    authentication_configuration {
      access_role_arn = aws_iam_role.kamekai_service_role.arn
    }

    image_repository {
      image_repository_type = "ECR"
      image_identifier      = "${aws_ecr_repository.kamekai.repository_url}:1.0.2-b96981db-dirty"

      image_configuration {
        port = "8080"

        runtime_environment_variables = {
          "APP_ENABLE_ANSI_LOGS" = "false"
        }
      }
    }
  }

  # See https://aws.amazon.com/apprunner/pricing/
  # Scroll down, there is a "Supported configurations" section.
  instance_configuration {
    cpu    = "0.25 vCPU"
    memory = "0.5 GB"

    instance_role_arn = aws_iam_role.kamekai.arn
  }

  health_check_configuration {
    path     = "/health"
    protocol = "HTTP"
  }

  # Enable auto deployment when new images are pushed
  auto_scaling_configuration_arn = aws_apprunner_auto_scaling_configuration_version.kamekai.arn
}

resource "aws_apprunner_auto_scaling_configuration_version" "kamekai" {
  auto_scaling_configuration_name = "kamekai-auto-scaling"

  max_concurrency = 5
  max_size        = 5
  min_size        = 1

  tags = {
    Name = "kamekai-auto-scaling"
  }
}