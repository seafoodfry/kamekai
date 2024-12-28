resource "aws_cloudwatch_log_group" "waf_logs" {
  name              = "/aws/waf/app-runner"
  retention_in_days = 3
}