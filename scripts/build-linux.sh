#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

OUT_DIR="${1:-binaries}"
mkdir -p "$OUT_DIR"
docker build --platform=linux/amd64 --target export --output "type=local,dest=${OUT_DIR}" -f Dockerfile .
echo "Exported Linux binaries to ${OUT_DIR}"
