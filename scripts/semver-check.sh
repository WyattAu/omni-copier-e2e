#!/usr/bin/env bash
# semver-checks against the latest tag. Skipped (with a note) before the
# first release, because there is no baseline to diff against yet.
set -euo pipefail
cd "$(dirname "$0")/.."
if ! git describe --tags --abbrev=0 >/dev/null 2>&1; then
  echo "semver-checks: no tags yet — nothing to diff against (skipped)"
  exit 0
fi
exec cargo semver-checks check --workspace
