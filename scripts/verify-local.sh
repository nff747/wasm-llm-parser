#!/usr/bin/env bash
# Local verification launcher for WASM Llm Parser
set -euo pipefail

echo "=========================================================="
echo " Running Local Verification for WASM Llm Parser"
echo "=========================================================="

if [ -f "tools/verify.mjs" ]; then
  node tools/verify.mjs
elif [ -d "tests" ]; then
  node --test tests/*.test.mjs
fi

echo "All tests verified successfully!"
