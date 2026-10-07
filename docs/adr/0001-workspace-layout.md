# 0001 — Cargo workspace layout (monorepo-first)

Date: 2026-10-04

## Status

Accepted

## Context

Estate convention (engineering-standards layers doc) splits crates into
L0 leaves, L1 substrate, L2 domain, and L3 composition. A template must
teach the shape, not just compile.

## Decision

- `crates/omni-core` — L0 leaf: zero workspace deps, `no_std`-capable via
  default-on `std` feature, REQ-tagged tests, criterion bench, fuzz target.
- `crates/omni-service` — L3 composition: axum + tokio + tracing binary with
  health/readiness and graceful shutdown.
- `crates/omni-cli` — clap binary composing on `omni-core`.
- `xtask/` — workspace task runner; Makefile delegates to it.
- `fuzz/` — cargo-fuzz harnesses (excluded from the workspace).
- Lint policy lives once in `[workspace.lints]`; crates inherit via
  `[lints] workspace = true`.

## Consequences

- Derived projects start with the estate's layer discipline demonstrated
  end-to-end; deleting example crates is the documented first step.
