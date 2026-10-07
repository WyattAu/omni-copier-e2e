# 0004 — Release flow: release-plz + vet + semver + attestation

Date: 2026-10-04

## Status

Accepted

## Context

Estate releases follow: bump → gates → semver-checks → dry-run → publish →
tag → registry verify (engineering-standards RELEASES.md). Manual repetition
of that flow does not survive ten crates.

## Decision

- release-plz opens the versioning/CHANGELOG PR on every push to main; its
  merge publishes (command: release) with `CARGO_REGISTRY_TOKEN`.
- Gates upstream of publish: tier-a CI, `cargo vet check`, semver-checks
  against the previous tag.
- Weekly `vet-refresh.yml` keeps imports.lock aligned with Dependabot waves.
- `scripts/reproducible-build.sh` provides the estate's determinism check
  (wired to `make repro` and the `repro` CI job since loop 3).

### Major dependency bumps are deliberate acts, not merges

A Dependabot PR that crosses a **major** version of a dependency with a vetted
audit trail is closed rather than merged, because "mergeable" and "safe" are
different questions:

- `criterion` 0.7 → 0.8 pulls a new crate tree into the workspace, so
  `cargo vet check` fails until the new imports are audited. Procedure:
  `./scripts/vet-refresh.sh` (regenerate imports) → `cargo vet` (review each
  new crate) → `cargo vet certify` → re-run `make bench` to confirm the perf
  gate still parses the new criterion output → then merge.
- The perf gate is the *input* contract for `criterion`'s output format, so a
  benchmark-tool bump is verified locally (`make bench` twice: baseline then
  gated) exactly like a re-baseline.

Routine patch/minor bumps merge through the normal path; the weekly
`vet-refresh.yml` keeps imports aligned in the meantime.

## Consequences

- Required secrets: `RELEASE_PLZ_TOKEN`, `CARGO_REGISTRY_TOKEN`.
- First release is the only manual-ish one: create tag `v0.1.0` after the
  first publish so semver-checks gains its baseline.
