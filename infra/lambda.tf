resource "aws_lambda_function" "api" {
  function_name = "kamekai"
  role          = aws_iam_role.kamekai.arn
  package_type  = "Image"
  image_uri     = "${aws_ecr_repository.kamekai.repository_url}:latest"

  memory_size = 128
  timeout     = 180 # 3 minutes

  reserved_concurrent_executions = 1 # -1 for unlimited.

  environment {
    variables = {
      RUST_BACKTRACE = "1"
    }
  }

  depends_on = [
    aws_cloudwatch_log_group.aws_iam_role.kamekai,
  ]
}
