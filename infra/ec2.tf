# Used the following query to find the latest AMI:
# ./run-cmd-in-shell.sh aws ec2 describe-images --region us-east-1 --owners 099720109477 --filters "Name=platform-details,Values=Linux/UNIX" "Name=architecture,Values=x86_64" "Name=creation-date,Values=2024-12-*" "Name=description,Values=*Ubuntu*" --query 'Images[?!contains(Description, `EKS`) && !contains(Description,`UNSUPPORTED`) && contains(Description, `"24.04"`) ]' > out.json
module "build_box" {
  count  = 0
  source = "./ec2s/linux/vanilla"

  name              = "build"
  ami               = "ami-079cb33ef719a7b78"
  type              = "t3.large"
  security_group_id = aws_security_group.ssh.id
  subnet_id         = module.vpc.public_subnets[0]
  ec2_key_name      = var.ec2_key_name

  instance_profile_name = aws_iam_instance_profile.kamekai_build_box.name
}