resource "aws_cloudwatch_log_group" "app_runner" {
  # The name must have the prefix "aws-waf" for it to work.connection {
  # Otherwise TF will throw weird errorsa about invalid ARNs.connection {
  # See
  # https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/wafv2_web_acl_logging_configuration#log_destination_configs-1
  name              = "aws-waf-logs-app-runner"
  retention_in_days = 3
}