output "app_runner_service_url" {
  value = aws_apprunner_service.kamekai.service_url
}

output "user_pool_endpoint" {
  value = aws_cognito_user_pool.kamekai.endpoint
}

output "user_pool_client_id" {
  value = aws_cognito_user_pool_client.desktop_client.id
}

output "build_box_public_dns" {
  value       = module.build_box[*].public_dns
  description = "Public dev DNS"
}