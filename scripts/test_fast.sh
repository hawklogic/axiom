#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2024 HawkLogic Systems

# Axiom fast test script
# Runs Rust tests only (no lints, no frontend tests)

set -e
cd "$(dirname "$0")/.."

echo "Running Rust tests..."
cargo test --workspace
echo "Tests passed."
