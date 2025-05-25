#!/usr/bin/env bash
set -e

IMAGE_NAME="aljama-dev"
CONTAINER_NAME="aljama-dev"
FORCE_CLEAN=false
REBUILD=false
RESET_VOLUMES=false

# Parse arguments
for arg in "$@"; do
  case $arg in
    --force-clean) FORCE_CLEAN=true ;;
    --rebuild) REBUILD=true ;;
    --reset) RESET_VOLUMES=true ;;
  esac
done

# Force clean if requested
if $FORCE_CLEAN; then
  echo "🧹 Removing image and container..."
  podman rm -f $CONTAINER_NAME || true
  podman rmi -f $IMAGE_NAME || true
fi

# Reset volumes if requested
if $RESET_VOLUMES; then
  echo "🗑️ Removing volumes..."
  podman volume rm cargo-cache rust-target || true
fi

# Build image if it doesn't exist or rebuild is requested
if $REBUILD || ! podman image exists $IMAGE_NAME; then
  echo "🔧 Building dev image..."
  podman build -f .devcontainer/Containerfile --target dev -t $IMAGE_NAME .
fi

# Create and start container if it doesn't exist
if ! podman ps -a --format "{{.Names}}" | grep -q "^${CONTAINER_NAME}$"; then
  echo "🚀 Starting container..."
  podman run -dit --name $CONTAINER_NAME \
    --init \
    -p 3000:3000 \
    -p 8080:8080 \
    --env-file .env \
    -v "$(pwd):/workspace:z" \
    -v cargo-cache:/root/.cargo \
    -v rust-target:/workspace/target \
    $IMAGE_NAME \
    bash
else
  # Start container if it's stopped
  if ! podman ps --format "{{.Names}}" | grep -q "^${CONTAINER_NAME}$"; then
    echo "▶️ Starting stopped container..."
    podman start $CONTAINER_NAME
  fi
fi

echo "🧬 Attaching to container..."
podman exec -it $CONTAINER_NAME bash
