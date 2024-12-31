resource "aws_iam_role" "cognito_sms_role" {
  name = "kamekai-cognito-sms"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "cognito-idp.amazonaws.com"
        }
        Condition = {
          StringEquals = {
            "sts:ExternalId" : "kamekai_cognito_sms" # Must match external_id in sms_configuration.
          }
        }
      }
    ]
  })
}

# IAM policy for SNS publish
resource "aws_iam_role_policy" "cognito_sns_policy" {
  name = "kamekai-cognito-sns-policy"
  role = aws_iam_role.cognito_sms_role.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "sns:Publish"
        ]
        Resource = "*"
      }
    ]
  })
}

resource "aws_cognito_user_pool" "kamekai" {
  name = "kamekai-users"

  account_recovery_setting {
    recovery_mechanism {
      name     = "verified_email"
      priority = 1
    }

    recovery_mechanism {
      name     = "verified_phone_number"
      priority = 2
    }
  }

  auto_verified_attributes = ["email"]
  username_attributes      = ["email"]

  device_configuration {
    challenge_required_on_new_device = true
  }


  # Setting this as ON will prevent passwordless login options.
  mfa_configuration          = "OPTIONAL"
  sms_authentication_message = "Your kamekai code is {####}"
  sms_configuration {
    external_id    = "kamekai_cognito_sms"
    sns_caller_arn = aws_iam_role.cognito_sms_role.arn
    sns_region     = data.aws_region.current.name
  }
  software_token_mfa_configuration {
    enabled = true
  }

  password_policy {
    minimum_length = 12
    # Requires advanced security features to be active.
    # See
    # https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pool-settings-threat-protection.html 
    password_history_size            = 6
    require_lowercase                = true
    require_numbers                  = true
    require_symbols                  = true
    require_uppercase                = true
    temporary_password_validity_days = 1
  }

  verification_message_template {
    default_email_option = "CONFIRM_WITH_CODE"
    email_subject        = "Verify your email for Kamekai"
    email_message        = "Your verification code is {####}"
    sms_message          = "Your kamekai code is {####}"
  }

  user_attribute_update_settings {
    attributes_require_verification_before_update = ["email"]
  }

  user_pool_add_ons {
    # See
    # https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pool-settings-threat-protection.html
    advanced_security_mode = "ENFORCED"
  }
}

# Cognito App Client for Desktop App.
resource "aws_cognito_user_pool_client" "desktop_client" {
  name         = "kamekai-desktop"
  user_pool_id = aws_cognito_user_pool.kamekai.id

  # We are using auth code w/ PKCE.
  generate_secret = false

  allowed_oauth_flows_user_pool_client = true
  allowed_oauth_flows                  = ["code"]
  allowed_oauth_scopes                 = ["openid", "email", "profile"]

  callback_urls = [
    "kamekai://auth/callback",            # Desktop app custom protocol.
    "http://localhost:1420/auth/callback" # Local development.
  ]

  logout_urls = [
    "kamekai://auth/logout",
    "http://localhost:1420/auth/logout"
  ]

  prevent_user_existence_errors = "ENABLED"
  supported_identity_providers  = ["COGNITO"]

  # Your app client accepts an IP address that your app adds to unauthenticated requests.
  # This IP address, and a device fingerprint, contribute to risk evaluation by Amazon Cognito
  # advanced security features. When you don't accept additional user context data, your app
  # client only accepts the device fingerprint.
  # Client Secret is required to set EnablePropagateAdditionalUserContextData as true
  #enable_propagate_additional_user_context_data = true
}

# # Cognito App Client for Web App (for future use)
# resource "aws_cognito_user_pool_client" "web_client" {
#   name         = "kamekai-web"
#   user_pool_id = aws_cognito_user_pool.kamekai_pool.id

#   generate_secret = true

#   allowed_oauth_flows                  = ["code"]
#   allowed_oauth_flows_user_pool_client = true
#   allowed_oauth_scopes                 = ["openid", "email", "profile"]

#   callback_urls = [
#     "https://app.kamekai.com/auth/callback", # Production web app
#     "http://localhost:3000/auth/callback"    # Local development
#   ]

#   logout_urls = [
#     "https://app.kamekai.com/auth/logout",
#     "http://localhost:3000/auth/logout"
#   ]

#   supported_identity_providers = ["COGNITO"]
# }

# Cognito Domain
resource "aws_cognito_user_pool_domain" "kamekai" {
  domain          = "auth.seafoodfry.ninja"
  user_pool_id    = aws_cognito_user_pool.kamekai.id
  certificate_arn = aws_acm_certificate.auth_cert.arn

  depends_on = [cloudflare_record.domain_root_dummy]
}