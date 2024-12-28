# Create IP set for allowed IPs.
resource "aws_wafv2_ip_set" "allowed_ips" {
  name               = "allowed-ips"
  description        = "IP set for allowed IPs"
  scope              = "REGIONAL"
  ip_address_version = "IPV4"

  addresses = [
    "${var.my_ip}/32",
  ]
}

# Associate WAF Web ACL with App Runner service.
resource "aws_wafv2_web_acl_association" "app_runner" {
  resource_arn = aws_apprunner_service.kamekai.arn
  web_acl_arn  = aws_wafv2_web_acl.app_runner_acl.arn
}

resource "aws_wafv2_web_acl" "app_runner_acl" {
  name        = "app-runner-ip-allowlist"
  description = "WAF Web ACL for App Runner IP whitelisting"
  scope       = "REGIONAL"

  default_action {
    block {}
  }

  rule {
    name     = "allow-specific-ips"
    priority = 1

    action {
      allow {}
    }

    statement {
      ip_set_reference_statement {
        arn = aws_wafv2_ip_set.allowed_ips.arn
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "AllowedIPsRule"
      sampled_requests_enabled   = true
    }
  }

  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "AppRunnerWAF"
    sampled_requests_enabled   = true
  }
}

# Optional: Add CloudWatch logging for WAF.
resource "aws_wafv2_web_acl_logging_configuration" "app_runner" {
  log_destination_configs = [aws_cloudwatch_log_group.app_runner.arn]
  resource_arn            = aws_wafv2_web_acl.app_runner_acl.arn
}

resource "aws_cloudwatch_log_resource_policy" "example" {
  policy_document = data.aws_iam_policy_document.example.json
  policy_name     = "webacl-policy-uniq-name"
}

data "aws_iam_policy_document" "example" {
  version = "2012-10-17"
  statement {
    effect = "Allow"
    principals {
      identifiers = ["delivery.logs.amazonaws.com"]
      type        = "Service"
    }
    actions   = ["logs:CreateLogStream", "logs:PutLogEvents"]
    resources = ["${aws_cloudwatch_log_group.app_runner.arn}:*"]
    condition {
      test     = "ArnLike"
      values   = ["arn:aws:logs:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:*"]
      variable = "aws:SourceArn"
    }
    condition {
      test     = "StringEquals"
      values   = [tostring(data.aws_caller_identity.current.account_id)]
      variable = "aws:SourceAccount"
    }
  }
}