variable "name" {
  type = string
}

variable "spot_max_price" {
  type    = string
  default = "0.6" # https://aws.amazon.com/ec2/spot/pricing/
}

variable "ami" {
  type = string
}

variable "type" {
  type = string
}

variable "security_group_id" {
  type = string
}

variable "subnet_id" {
  type = string
}

variable "ec2_key_name" {
  type = string
}

variable "volume_size" {
  type    = number
  default = 10
}

variable "instance_profile_name" {
  type = string
}

variable "install_docker" {
  type    = bool
  default = true
}

variable "install_rust" {
  type    = bool
  default = true
}

variable "install_aws_cli" {
  type    = bool
  default = true
}