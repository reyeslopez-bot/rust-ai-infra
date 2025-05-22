#!/bin/bash

# 🚀 AI Infra Dev Runner
set -e

IMAGE_NAME="ai-infra-dev"
CONTAINER_NAME="ai-infra"
APP_PORT=8000

# 🛠 Build the container image
echo "📦 Building container image..."
podman build -t $IMAGE_NAME -f .devcontainer/Containerfile .

# 🧠 Run the container with mounted volume and .env variables
echo "🐳 Running container with .env and /workspace mount..."
podman run -it --rm \
  --name $CONTAINER_NAME \
  --env-file .env \
  -v "$(pwd)":/workspace:Z \
  -w /workspace \
  -p $APP_PORT:$APP_PORT \
  $IMAGE_NAME bash

