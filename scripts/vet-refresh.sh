#!/usr/bin/env bash
# Regenerate cargo-vet imports after a Dependabot wave. CI (vet-refresh.yml)
# opens the PR; humans review it like any other.
#
# cargo-vet >= 0.10 removed `fix-imports` (import fixing is part of
# `regenerate imports`), so this script sticks to the stable subcommand.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo vet regenerate imports
