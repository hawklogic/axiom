#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2024 HawkLogic Systems

# Verify all source files have SPDX headers
# Required for Apache-2.0 compliance

set -e
cd "$(dirname "$0")/.."

MISSING=0
CHECKED=0

echo "Checking SPDX headers..."

for ext in rs ts svelte css; do
  while IFS= read -r -d '' f; do
    CHECKED=$((CHECKED+1))
    if ! head -2 "$f" | grep -q "SPDX-License-Identifier: Apache-2.0"; then
      echo "  Missing: $f"
      MISSING=$((MISSING+1))
    fi
  done < <(find . -name "*.$ext" -not -path "./node_modules/*" -not -path "./target/*" -not -path "./.git/*" -print0 2>/dev/null)
done

if [ $CHECKED -eq 0 ]; then
  echo "No source files found to check."
  exit 0
fi

if [ $MISSING -gt 0 ]; then
  echo ""
  echo "ERROR: $MISSING of $CHECKED files missing SPDX headers"
  echo "Required header:"
  echo "  // SPDX-License-Identifier: Apache-2.0"
  echo "  // Copyright 2024 HawkLogic Systems"
  exit 1
fi

echo "All $CHECKED source files have SPDX headers."
