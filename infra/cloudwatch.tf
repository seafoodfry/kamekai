resource "aws_cloudwatch_log_group" "kamekai" {
  name              = "/aws/lambda/kamekai"
  retention_in_days = 7
}