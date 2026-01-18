#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2024 HawkLogic Systems

# Axiom production build script
# Builds the complete application for distribution

set -e
cd "$(dirname "$0")/.."

echo "=== Axiom Build ==="
echo ""

# Check SPDX headers
echo "[1/4] Checking SPDX headers..."
./scripts/check_spdx.sh

# Build Rust workspace
echo "[2/4] Building Rust workspace..."
cargo build --workspace --release

# Build frontend
echo "[3/4] Building frontend..."
npm run build

# Build Tauri bundle
echo "[4/4] Building Tauri bundle..."
npm run tauri build

echo ""
echo "=== Build Complete ==="
echo "Output: src-tauri/target/release/bundle/"
