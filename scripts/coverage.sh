#!/usr/bin/env bash
# llvm-cov gate — tier a threshold (>=90%). Override: ./scripts/coverage.sh 80
set -euo pipefail
cd "$(dirname "$0")/.."
exec cargo llvm-cov --workspace --all-features --summary-only --fail-under "${1:-90}"
