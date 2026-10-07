#!/usr/bin/env bash
# Non-nix fallback: print the manual toolchain setup for this repo.
# The canonical environment is the flake (`.envrc`) or the devcontainers.
set -euo pipefail
cat <<'MSG'
Manual toolchain (no nix):
  1. rustup (https://rustup.rs) — the repo's rust-toolchain.toml pins stable
     and installs rustfmt/clippy/rust-src automatically.
  2. Gate tools:
       cargo install --locked cargo-nextest cargo-llvm-cov cargo-deny cargo-vet
  3. Then: make ci
Prefer zero setup? Open the repo in a devcontainer (image flavor), or use
nix: `nix develop`.
MSG
