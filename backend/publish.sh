#!/bin/sh
set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 IMAGE VERSION"
    echo "The IMAGE is what was built via 'make build'."
    echo "The VERSION should match the version in Cargo.toml."
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
echo "   - ${LATEST_URI}"