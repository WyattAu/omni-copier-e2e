# Thin wrapper over scripts/ and cargo — the same verbs everywhere. CI runs
# these same commands via the estate reusable workflow (tier a).

.PHONY: bench bench-update repro build test lint fmt fmt-check coverage mutants vet semver docs contract ci clean

build:
	cargo build --workspace --all-features --locked

test:
	cargo nextest run --workspace --all-features --locked

lint:
	cargo clippy --workspace --all-features --locked -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all --check

coverage:
	./scripts/coverage.sh

mutants:
	./scripts/mutants.sh

vet:
	cargo vet check --all-features

semver:
	./scripts/semver-check.sh

docs:
	cargo doc --workspace --no-deps --all-features

contract:
	./scripts/check-contract.sh

## What CI gates before merge (mirror of .github/workflows/ci.yml):
ci: contract fmt-check lint test vet

repro:
	./scripts/repro-check.sh

bench:
	./scripts/bench-budget.sh

bench-update:
	./scripts/bench-budget.sh --update

clean:
	cargo clean
