# OmniRust-template

Maximalist Rust **monorepo** template: cargo workspace + estate tier-a gates
(clippy pedantic, loom, miri, llvm-cov ≥90%, cargo-deny, cargo-vet, mutation
testing, fuzzing) + full IDE/OS integration (nix flake, dual devcontainers,
VS Code) — part of the [WyattAu Omni template family](https://github.com/WyattAu?tab=repositories&q=omni-).

## Start here (after "Use this template")

1. Rename: `omni-core` / `omni-service` / `omni-cli` → your crate names
   (repo name = first crate name, estate naming rule), and replace the
   example parser in `crates/omni-core/src/text.rs` with your domain.
2. Pick a door — all three resolve to identical toolchains:

   | Door | Command |
   |---|---|
   | nix + direnv (host) | `direnv allow` |
   | Devcontainer (image) | VS Code → *Reopen in Container* |
   | Devcontainer (nix)  | palette → *Rebuild in Container* → pick `.devcontainer/nix/` |

   No nix, no docker? `./scripts/bootstrap.sh` prints the manual path.
3. `make ci` — must be green before your first push. CI runs exactly this.

## Make targets

| Target | Gate |
|---|---|
| `make build` | `cargo build --workspace --all-features --locked` |
| `make test` | `cargo nextest run --workspace --all-features --locked` |
| `make lint` | clippy, tier-a posture (`-D warnings`, pedantic, panic family denied) |
| `make fmt` / `fmt-check` | rustfmt |
| `make coverage` | llvm-cov, ≥90% lines (process surfaces excluded — ADR-0005) |
| `make vet` | cargo-vet supply-chain check |
| `make semver` | semver-checks vs latest tag |
| `make mutants` | cargo-mutants (weekly policy, MUTATION.md) |
| `make contract` | Omni Core Contract structural checks |
| `make ci` | contract + fmt-check + lint + test + vet |

## What is inside

```
crates/omni-core      L0 leaf: no_std-capable, REQ-tagged, proptest, bench, fuzz target
crates/omni-service   L3 example: axum, health/ready, graceful shutdown
crates/omni-cli       example clap binary composing on omni-core
xtask/                workspace task runner (coverage/mutants/vet/semver/contract)
fuzz/                 cargo-fuzz harnesses
supply-chain/         cargo-vet (config + audits; imports.lock via cargo vet)
deny.toml             dependency governance (licenses/advisories/bans)
docs/adr/             decision log (start at 0000)
.github/workflows/    ci (tier a + msrv + contract), release, docs, mutation, vet-refresh, devcontainers
.forgejo/             thin self-hosted mirror (scripts are canonical)
```

## CI gates (tier a)

| Gate | Threshold |
|---|---|
| build/test `--locked` (all-features + no-default-features) | ✅ |
| clippy `-D warnings` + pedantic + unwrap/indexing/panic denied | ✅ |
| llvm-cov | ≥ 90% lines (process surfaces excluded via `coverage-ignore-regex`, ADR-0005) |
| loom model checking (`--cfg loom`) | ✅ |
| miri (nightly) | ✅ |
| cargo-deny (advisories/licenses/bans) | ✅ |
| cargo-vet | ✅ (weekly auto-refresh) |
| cargo-fuzz | harnesses provided |
| MSRV | 1.85 (dedicated CI leg) |
| semver-checks | on version diff |
| mutation testing | weekly, promotes to gate per policy |
| devcontainers | both flavors built in CI |

## Optional toggles (documented patterns, add the crate when needed)

- **web** — add `crates/omni-web` (Leptos + Trunk); enable the shared
  workflow's `wasm: true` input for the `wasm32-unknown-unknown` check.
- **embedded** — add a no_std crate + set `embedded: true` for the
  `thumbv7em-none-eabihf` check (pattern: ADR-0003).
- **service** — extend `omni-service` with the estate stacks (axum-stack,
  outbox-kit, healthkit, telemetry-init).

## Release flow

release-plz PRs version bumps + CHANGELOG on push to main; merging publishes
to crates.io. Secrets needed: `RELEASE_PLZ_TOKEN`, `CARGO_REGISTRY_TOKEN`
(ADR-0004). Before the first release there is nothing to do — semver-checks
skips until the first tag exists.

## Estate pointers

- Gates, policies, layer model: [engineering-standards](https://github.com/WyattAu/engineering-standards)
- Omni Core Contract: [OMNI-CORE.md](https://github.com/WyattAu/engineering-standards/blob/main/OMNI-CORE.md)

## License

Apache-2.0 — commercial use expressly permitted.


## Performance budgets

Performance is a gate, not a hope. `make bench` measures, writes
`bench/current.tsv`, and compares it against the committed
`bench/baseline.tsv`; anything more than the threshold worse fails. The
comparator (`scripts/compare-bench.py`) is identical across the whole Omni
estate, so the policy is auditable in one place.

| Verb | What it does |
|---|---|
| `make bench` | measure + compare (advisory job in CI: `perf`) |
| `make bench-update` | deliberately re-baseline; the only way a baseline moves |

The first run on a fresh clone records the baseline instead of failing, so the
gate is meaningful from the second run onwards. Override the budget per run
with `OMNI_BENCH_THRESHOLD_PCT=15 make bench`. Rationale and per-template
metrics: `docs/adr/0007-performance-budget-gate.md`.


## Determinism

`make repro` builds twice from a clean state with a pinned `SOURCE_DATE_EPOCH`
and compares artifact hashes. Toolchains that are deterministic gate the build;
toolchains that embed timestamps or build ids by design report the difference
and explain why, rather than pretending to be reproducible. Rationale and the
per-toolchain split: `docs/adr/0008-determinism-verification.md`.

Copier update channel verified end-to-end (loop 12).
