# 0005 — Coverage policy for process surfaces

Date: 2026-10-04

## Status

Accepted

## Context

The estate's tier-a gate is `cargo llvm-cov --fail-under-lines 90` across
the workspace (rust-kit.yml keeps the command simple). A template shipping
runnable binaries hits two LLVM profile realities:

1. A child process stopped via `kill()` never writes its profile — the
   spawned service's `main` reads 0% no matter how good the smoke test is.
2. `xtask`/CLI process wrappers shell out to external tools; their value is
   orchestration, not algorithmic logic. (`#[coverage(off)]` would be the
   per-item answer but is still unstable.)

## Decision

- rust-kit.yml grew a `coverage-ignore-regex` input (single-token
  `--ignore-filename-regex=` form, quote-safe). This template passes
  `xtask/src|omni-service/src/main.rs` — the process surfaces.
- Everything else stays under the ≥90% line gate: `omni-core` and
  `omni-cli`'s logic are lib-wrapped or spawn-tested with natural child
  exit, so their profiles count.
- The workspace bar stays at 90 — demo scaffolding must never force the
  template to weaken the estate's numbers.

## Consequences

- Coverage measures logic, not OS plumbing; mutation testing concentrates
  on the same surface.
- Derived projects adding real service logic should put it in a lib module
  with in-process tests (see ADR-0001's L3 crate notes) and shrink the
  ignore-regex as they go.
