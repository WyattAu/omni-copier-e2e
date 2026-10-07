#!/usr/bin/env bash
# Weekly mutation run (MUTATION.md policy): continue-on-error until survivors
# are zero across 4 consecutive weekly runs, then graduate to a gate.
set -euo pipefail
cd "$(dirname "$0")/.."
exec cargo mutants --all-features -o mutants-out "$@"
