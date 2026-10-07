#!/usr/bin/env bash
# Omni Core Contract checks (THREAT-MODEL T4/T5):
#   1. Every gate the Makefile exposes maps to a script that exists.
#   2. Every workflow declares an explicit least-privilege permissions block.
#   3. The shared reusable-workflow call is present.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0

for wf in .github/workflows/*.yml .forgejo/workflows/*.yml; do
  grep -q 'permissions:' "$wf" || { echo "FAIL: $wf missing explicit permissions block"; fail=1; }
done

grep -q 'rust-kit.yml' .github/workflows/ci.yml || { echo "FAIL: ci.yml does not call the estate reusable workflow"; fail=1; }

while read -r script; do
  [ -x "$script" ] || { echo "FAIL: Makefile references missing script $script"; fail=1; }
done < <(grep -oP '(?<=^	)(scripts/[a-z-]+\.sh)' Makefile | sort -u)

[ "$fail" -eq 0 ] && echo "contract: OK"
exit "$fail"
