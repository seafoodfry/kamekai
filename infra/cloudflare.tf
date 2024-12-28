data "cloudflare_zone" "domain" {
  name = "seafoodfry.ninja"
}

# Create DNS validation records in Cloudflare instead of Route53.
resource "cloudflare_record" "cert_validation" {
  for_each = {
    for dvo in aws_acm_certificate.api.domain_validation_options : dvo.domain_name => {
      name  = dvo.resource_record_name
      value = dvo.resource_record_value
      type  = dvo.resource_record_type
    }
  }

  zone_id = data.cloudflare_zone.domain.id
  name    = each.value.name
  content = each.value.value
  type    = each.value.type
  ttl     = 60
  proxied = false
}

resource "aws_acm_certificate_validation" "api" {
  certificate_arn         = aws_acm_certificate.api.arn
  validation_record_fqdns = [for record in cloudflare_record.cert_validation : record.hostname]
}


resource "aws_apprunner_custom_domain_association" "api" {
  domain_name = "api.seafoodfry.ninja"
  service_arn = aws_apprunner_service.kamekai.arn

  depends_on = [
    aws_acm_certificate_validation.api
  ]
}

# # Create App Runner validation records in Cloudflare.
# resource "cloudflare_record" "apprunner_validation" {
#   for_each = {
#     for record in aws_apprunner_custom_domain_association.api.certificate_validation_records : record.name => {
#       type  = record.type
#       value = record.value
#     }
#   }

#   zone_id = data.cloudflare_zone.domain.id
#   name    = each.key
#   content = each.value.value
#   type    = each.value.type
#   ttl     = 300
#   proxied = false
# }

# # Create CNAME record for the App Runner domain in Cloudflare.
# resource "cloudflare_record" "app_runner_domain" {
#   zone_id         = data.cloudflare_zone.domain.id
#   allow_overwrite = true
#   name            = "api"
#   type            = "CNAME"
#   content         = aws_apprunner_custom_domain_association.api.dns_target
#   ttl             = 60 # Must be 1 when proxied is enabled.
#   # Do NOT turn it on, app runner and cloudflare will fight for HTTPS control and
#   # you'll end with a TON of redirects.
#   proxied = false
#   comment = "App Runner domain"

#   depends_on = [
#     aws_apprunner_custom_domain_association.api,
#     cloudflare_record.apprunner_validation
#   ]
# }