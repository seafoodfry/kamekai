#!/bin/sh
#
#  Image from https://aws.amazon.com/getting-started/guides/deploy-webapp-apprunner/
#  docker buildx build --platform linux/amd64 -t nginx-web-app -f Dockerfile.nginx . --provenance=false
#  ./run-cmd-in-shell.sh ./publish.sh nginx-web-app:latest 0.1.3
set -euo pipefail


AWS_REGION="${AWS_REGION:-us-east-1}"
AWS_ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)
REPO_URI="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/kamekai"

# The --dirty flag will append -dirty if there are uncommitted changes
CRATE_VERSION=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[] | select(.name == "backend") | .version')
GIT_HASH=$(git describe --always --dirty --abbrev=8)
IMG_TAG="${CRATE_VERSION}-${GIT_HASH}"

IMG_URI="${REPO_URI}:${IMG_TAG}"

echo "📦 Version: ${CRATE_VERSION}"
echo "🔨 Git hash: ${GIT_HASH}"
echo "🚀 Publishing to ${IMG_URI}..."

aws ecr get-login-password --region ${AWS_REGION} | docker login --username AWS --password-stdin "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"

# For some ctx on why --provenance=false is used, see:
# https://stackoverflow.com/questions/75131872/error-failed-to-solve-failed-commit-on-ref-unexpected-status-400-bad-reques
# https://github.com/aws/containers-roadmap/issues/1596
# https://aws.amazon.com/blogs/containers/oci-artifact-support-in-amazon-ecr/
# Also,
# https://blog.jaimyn.dev/how-to-build-multi-architecture-docker-images-on-an-m1-mac/
#
# NOTE: app runner doesn't seem to support non-amd64 images yet.
# Haven't found docs saying so. But it's not working.
set -x
docker buildx build --platform linux/amd64 -f Dockerfile -t "${IMG_URI}" --no-cache --provenance=false . --push
set +x

echo "✅ Successfully published:"
echo "   - ${IMG_URI}"