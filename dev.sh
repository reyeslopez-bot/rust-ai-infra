#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

IMAGE_NAME="aljama-dev"
CONTAINER_NAME="aljama-dev"
FORCE_CLEAN=false
REBUILD=false
RESET_VOLUMES=false

# Parse CLI args
for arg in "$@"; do
  case $arg in
    --force-clean) FORCE_CLEAN=true ;;
    --rebuild) REBUILD=true ;;
    --reset) RESET_VOLUMES=true ;;
    *) echo "❌ Unknown option: $arg" && exit 1 ;;
  esac
done

# --- Cleanup phase ---
if $FORCE_CLEAN; then
  echo "🧹 Removing container and image..."

  if podman ps -a --format '{{.Names}}' | grep -Fxq "$CONTAINER_NAME"; then
    podman rm -f "$CONTAINER_NAME"
  fi

  if podman images --format '{{.Repository}}:{{.Tag}}' | grep -Fxq "${IMAGE_NAME}:latest"; then
    podman rmi -f "$IMAGE_NAME"
  fi
fi

if $RESET_VOLUMES; then
  echo "🗑️ Removing volumes..."
  podman volume rm cargo-cache rust-target pnpm-store || true
fi

# --- Copy lockfile to bust Docker cache ---
cp pnpm-lock.yaml .devcontainer/pnpm-lock.yaml 2>/dev/null || true

# --- Load .env and extract DATABASE_URL ---
if [[ -f .env ]]; then
  export $(grep DATABASE_URL .env | xargs)
else
  echo "⚠️  No .env file found. DATABASE_URL may be missing during build."
fi

# --- Build image if missing or --rebuild ---
if $REBUILD || ! podman images --format '{{.Repository}}:{{.Tag}}' | grep -Fxq "${IMAGE_NAME}:latest"; then
  echo "🔧 Building dev image..."
  podman build -f .devcontainer/Containerfile \
    --target dev \
    --build-arg DATABASE_URL="${DATABASE_URL:-}" \
    -t "$IMAGE_NAME" .
fi

# --- Prepare optional env file mount ---
ENV_ARGS=()
[[ -f .env ]] && ENV_ARGS+=(--env-file .env)

# --- Run container if it doesn't exist ---
if ! podman ps -a --format '{{.Names}}' | grep -Fxq "$CONTAINER_NAME"; then
  echo "🚀 Creating and starting container..."
  podman run -dit \
    --name "$CONTAINER_NAME" \
    --init \
    -p 3000:3000 \
    -p 8080:8080 \
    "${ENV_ARGS[@]}" \
    -v "$(pwd):/workspace:z" \
    -v cargo-cache:/root/.cargo \
    -v rust-target:/workspace/target \
    -v pnpm-store:/root/.pnpm-store \
    --add-host=host.containers.internal:host-gateway
    "$IMAGE_NAME" \
    bash
elif ! podman ps --format '{{.Names}}' | grep -Fxq "$CONTAINER_NAME"; then
  echo "▶️ Starting existing container..."
  podman start "$CONTAINER_NAME"
fi

# --- Final log and attach ---
echo ""
echo "✅ Dev container is up and running."
echo "📁 Project mounted at: /workspace"
echo "🔧 Ports forwarded: 3000 → localhost:3000, 8080 → localhost:8080"
echo "💾 Volumes: cargo-cache, rust-target, pnpm-store"
echo "🔄 Auto-attaching to container shell..."

exec podman exec -it "$CONTAINER_NAME" bash

