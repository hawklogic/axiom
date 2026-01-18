#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2024 HawkLogic Systems

# Axiom full test script
# Runs all tests, lints, and checks

set -e
cd "$(dirname "$0")/.."

echo "=== Axiom Full Test Suite ==="
echo ""

# Check SPDX headers
echo "[1/5] Checking SPDX headers..."
./scripts/check_spdx.sh

# Rust tests
echo "[2/5] Running Rust tests..."
cargo test --workspace

# Rust lints
echo "[3/5] Running Clippy..."
cargo clippy --workspace -- -D warnings

# Frontend type check
echo "[4/5] Checking frontend types..."
npm run check

# Frontend tests
echo "[5/5] Running frontend tests..."
npm run test

echo ""
echo "=== All Tests Passed ==="
