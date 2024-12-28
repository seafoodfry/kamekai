data "aws_partition" "current" {}

variable "my_ip" {
  type      = string
  sensitive = true
}