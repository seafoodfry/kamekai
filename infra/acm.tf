# See
# https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/acm_certificate_validation
# and
# https://github.com/terraform-aws-modules/terraform-aws-app-runner/blob/d1f2fe7e3f83480def146d95fd7c5c9502018572/main.tf#L436
# and
# https://github.com/hashicorp/terraform-provider-aws/issues/23460
# and
# https://github.com/hashicorp/terraform-provider-aws/issues/14447#issuecomment-668766123
data "aws_route53_zone" "domain" {
  name = "seafoodfry.ninja"
}

resource "aws_acm_certificate" "api" {
  domain_name       = "api.seafoodfry.ninja"
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }
}

# resource "aws_route53_record" "cert_validation" {
#   for_each = {
#     for dvo in aws_acm_certificate.api.domain_validation_options : dvo.domain_name => {
#       name   = dvo.resource_record_name
#       record = dvo.resource_record_value
#       type   = dvo.resource_record_type
#     }
#   }

#   allow_overwrite = true
#   name            = each.value.name
#   records         = [each.value.record]
#   ttl             = 60
#   type            = each.value.type
#   zone_id         = data.aws_route53_zone.domain.zone_id
# }

# resource "aws_acm_certificate_validation" "api" {
#   certificate_arn         = aws_acm_certificate.api.arn
#   validation_record_fqdns = [for record in aws_route53_record.cert_validation : record.fqdn]
# }

# resource "aws_apprunner_custom_domain_association" "api" {
#   domain_name = "api.seafoodfry.ninja"
#   service_arn = aws_apprunner_service.kamekai.arn

#   depends_on = [
#     aws_acm_certificate_validation.api
#   ]
# }

# # Create validation records required by App Runner.
# resource "aws_route53_record" "apprunner_validation" {
#   for_each = {
#     for record in aws_apprunner_custom_domain_association.api.certificate_validation_records : record.name => {
#       type  = record.type
#       value = record.value
#     }
#   }

#   allow_overwrite = true
#   zone_id         = data.aws_route53_zone.domain.zone_id
#   name            = each.key
#   type            = each.value.type
#   ttl             = 300
#   records         = [each.value.value]
# }

# # xref:
# # https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/apprunner_hosted_zone_id
# data "aws_apprunner_hosted_zone_id" "main" {}

# # Create ALIAS record for the domain - provides better performance than CNAME.
# resource "aws_route53_record" "app_runner_domain" {
#   zone_id = data.aws_route53_zone.domain.zone_id
#   name    = "api.seafoodfry.ninja"
#   type    = "A" # We use type "A" even though it's an ALIAS.

#   alias {
#     # The App Runner DNS target becomes the ALIAS target.
#     name                   = aws_apprunner_custom_domain_association.api.dns_target
#     zone_id                = data.aws_apprunner_hosted_zone_id.main.id
#     evaluate_target_health = true
#   }

#   depends_on = [
#     aws_apprunner_custom_domain_association.api,
#     aws_route53_record.apprunner_validation
#   ]
# }