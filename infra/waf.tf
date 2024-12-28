resource "aws_wafv2_ip_set" "allowed_ips" {
  name               = "allowed-ips"
  description        = "IP addresses allowed to access the App Runner service"
  scope              = "REGIONAL"
  ip_address_version = "IPV4"

  addresses = [
    var.my_ip,
  ]
}

# See https://docs.aws.amazon.com/waf/latest/developerguide/waf-rule-action.html
resource "aws_wafv2_web_acl" "app_runner" {
  name        = "app-runner-protection"
  description = "WAF rules to protect App Runner service"
  scope       = "REGIONAL"

  # By default, block all traffic.
  default_action {
    block {}
  }

  # Rule to allow specific IPs.
  rule {
    name     = "AllowSpecificIPs"
    priority = 1
    override_action {
      # If the rule's conditions match (like matching an IP in our allowed IP set),
      # the rule's intrinsic "allow" nature takes effect.
      none {}
    }
    statement {
      ip_set_reference_statement {
        arn = aws_wafv2_ip_set.allowed_ips.arn
      }
    }
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "AppAllowedIPMetric"
      sampled_requests_enabled   = true
    }
  }

  # Rule to check for custom header.
  rule {
    name     = "RequireCustomHeader"
    priority = 2
    override_action {
      # If the rule's conditions match (like matching an IP in our allowed IP set),
      # the rule's intrinsic "allow" nature takes effect.
      none {}
    }
    statement {
      byte_match_statement {
        search_string         = "your-secret-value"
        positional_constraint = "EXACTLY"
        field_to_match {
          single_header {
            name = "X-Custom-Auth"
          }
        }
        text_transformation {
          priority = 1
          type     = "NONE"
        }
      }
    }
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "AppCustomHeaderMetric"
      sampled_requests_enabled   = true
    }
  }

  # Rule to rate limit requests per IP.
  rule {
    name     = "RateLimitPerIP"
    priority = 3
    override_action {
      # If the rule's conditions match (like matching an IP in our allowed IP set),
      # the rule's intrinsic "allow" nature takes effect.
      none {}
    }
    statement {
      rate_based_statement {
        limit              = 10 # Requests per 5 minutes.
        aggregate_key_type = "IP"
      }
    }
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "AppRateLimitMetric"
      sampled_requests_enabled   = true
    }
  }
  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "AppRunnerWAFMetric"
    sampled_requests_enabled   = true
  }
}

# Associate the Web ACL with your App Runner service.
resource "aws_wafv2_web_acl_association" "app_runner" {
  resource_arn = aws_apprunner_service.kamekai.arn
  web_acl_arn  = aws_wafv2_web_acl.app_runner.arn
}

resource "aws_wafv2_web_acl_logging_configuration" "app_runner" {
  log_destination_configs = [aws_cloudwatch_log_group.waf_logs.arn]
  resource_arn            = aws_wafv2_web_acl.app_runner.arn
}