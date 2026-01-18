#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2024 HawkLogic Systems

# Axiom development server
# Runs Tauri in development mode with hot reload

set -e
cd "$(dirname "$0")/.."

echo "Starting Axiom in development mode..."
npm run tauri dev
