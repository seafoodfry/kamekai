#!/bin/sh
#
# Tested it with:
#
#  docker run -it -p 8080:80 nginx:1.27
#  ./run-cmd-in-shell.sh ./publish.sh nginx:1.27 0.1.1
#
#  docker pull --platform linux/amd64 nginx:1.27.3
#  ./run-cmd-in-shell.sh ./publish.sh nginx:1.27.3 0.1.2
#
#  Image from https://aws.amazon.com/getting-started/guides/deploy-webapp-apprunner/
#  docker buildx build --platform linux/amd64 -t nginx-web-app -f Dockerfile.nginx . --provenance=false
#  ./run-cmd-in-shell.sh ./publish.sh nginx-web-app:latest 0.1.3
set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 container:tag version"
    echo "The IMAGE is what was built via 'make build'."
    echo "The VERSION is the version of the crate."
    exit 1
fi

# The image and version are set in the Makefile and passed as arguments.
IMG="$1"
VERSION="$2"

# Get the short commit hash (first 8 characters)
# The --dirty flag will append -dirty if there are uncommitted changes
GIT_HASH=$(git describe --always --dirty --abbrev=8)
FULL_VERSION="${VERSION}-${GIT_HASH}"

AWS_REGION="${AWS_REGION:-us-east-1}"
AWS_ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)
REPO_URI="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/kamekai"
IMG_URI="${REPO_URI}:${FULL_VERSION}"

echo "📦 Version: ${VERSION}"
echo "🔨 Git hash: ${GIT_HASH}"
echo "🚀 Publishing ${IMG} to ${IMG_URI}..."

aws ecr get-login-password --region ${AWS_REGION} | docker login --username AWS --password-stdin "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
docker tag "${IMG}" "${IMG_URI}"
docker push "${IMG_URI}"

echo "✅ Successfully published:"
echo "   - ${IMG_URI}"