################
# Service role #
################
resource "aws_iam_role" "kamekai_service_role" {
  name               = "kamekai-service-role"
  assume_role_policy = data.aws_iam_policy_document.kamekai_service_role.json
}

data "aws_iam_policy_document" "kamekai_service_role" {
  statement {
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["build.apprunner.${data.aws_partition.current.dns_suffix}"]
    }
    actions = ["sts:AssumeRole"]
  }
}

#####################
# ECR access policy #
#####################
resource "aws_iam_role_policy_attachment" "access" {
  role       = aws_iam_role.kamekai_service_role.name
  policy_arn = aws_iam_policy.access.arn
}

resource "aws_iam_policy" "access" {
  name        = "kamekai-ecr-access"
  description = "IAM policy for the kamekai backend for AWS App Runner"
  policy      = data.aws_iam_policy_document.access.json
}

data "aws_iam_policy_document" "access" {
  statement {
    sid = "ReadPrivateEcr"
    actions = [
      "ecr:BatchCheckLayerAvailability",
      "ecr:BatchGetImage",
      "ecr:DescribeImages",
      "ecr:GetDownloadUrlForLayer",
    ]
    resources = [aws_ecr_repository.kamekai.arn]
  }

  statement {
    sid = "AuthPrivateEcr"
    actions = [
      "ecr:DescribeImages",
      "ecr:GetAuthorizationToken",
    ]
    resources = ["*"]
  }
}

#################
# Instance role #
#################
resource "aws_iam_role" "kamekai" {
  name               = "kamekai"
  assume_role_policy = data.aws_iam_policy_document.trust.json
}

data "aws_iam_policy_document" "trust" {
  statement {
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["tasks.apprunner.${data.aws_partition.current.dns_suffix}"]
    }
    actions = ["sts:AssumeRole"]
  }
}

#################$$
# App permissions #
#################$$
resource "aws_iam_role_policy_attachment" "kamekai" {
  role       = aws_iam_role.kamekai.name
  policy_arn = aws_iam_policy.kamekai.arn
}

resource "aws_iam_policy" "kamekai" {
  name        = "kamekai"
  description = "IAM policy for the kamekai backend"
  policy      = data.aws_iam_policy_document.kamekai.json
}

data "aws_iam_policy_document" "kamekai" {
  # https://docs.aws.amazon.com/service-authorization/latest/reference/list_awssecuritytokenservice.html
  statement {
    effect = "Allow"
    actions = [
      "sts:GetCallerIdentity"
    ]
    resources = ["*"]
  }

  # https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrock.html
  statement {
    effect = "Allow"
    actions = [
      "bedrock:InvokeModel",
      "bedrock:InvokeModelWithResponseStream",
    ]
    resources = ["*"]
  }
}

#################
# EC2 Build Box #
#################
resource "aws_iam_instance_profile" "kamekai_build_box" {
  name = "kamekai-build-box"
  role = aws_iam_role.kamekai_build_box.name
}

resource "aws_iam_role" "kamekai_build_box" {
  name               = "kamekai-build-box"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.kamekai_build_box_trust.json
}

data "aws_iam_policy_document" "kamekai_build_box_trust" {
  statement {
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_policy" "kamekai_build_box" {
  name        = "kamekai-build-box"
  description = "Build box for Kamekai"
  policy      = data.aws_iam_policy_document.kamekai_build_box.json
}


resource "aws_iam_role_policy_attachment" "kamekai_build_box" {
  role       = aws_iam_role.kamekai_build_box.name
  policy_arn = aws_iam_policy.kamekai_build_box.arn
}

data "aws_iam_policy_document" "kamekai_build_box" {
  statement {
    sid = "ReadPrivateEcr"
    actions = [
      "ecr:CompleteLayerUpload",
      "ecr:UploadLayerPart",
      "ecr:InitiateLayerUpload",
      "ecr:BatchCheckLayerAvailability",
      "ecr:PutImage",
      "ecr:BatchGetImage",
      "ecr:DescribeImages",
      "ecr:GetDownloadUrlForLayer",
    ]
    resources = [aws_ecr_repository.kamekai.arn]
  }

  statement {
    sid = "AuthPrivateEcr"
    actions = [
      "ecr:GetAuthorizationToken",
    ]
    resources = ["*"]
  }
}