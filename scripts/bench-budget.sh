#!/usr/bin/env bash
# Perf budget gate: measure, emit bench/current.tsv, compare to the committed
# baseline with the shared comparator. `make bench-update` re-baselines
# deliberately - it is the only way a baseline moves.
set -euo pipefail
cd "$(dirname "$0")/.."
BASELINE=bench/baseline.tsv
CURRENT=bench/current.tsv
THRESHOLD_PCT="${OMNI_BENCH_THRESHOLD_PCT:-10}"
UPDATE=()
[ "${1:-}" = "--update" ] && UPDATE=(--update)
mkdir -p bench

# Criterion owns the statistics; we own the policy. Bench targets are resolved
# from cargo metadata because `cargo bench --workspace --benches -- <args>`
# also hands the arguments to the lib test harnesses, which reject them.
mapfile -t bench_targets < <(
  cargo metadata --no-deps --format-version 1 --offline 2>/dev/null |
    python3 -c 'import json, sys
meta = json.load(sys.stdin)
for pkg in meta["packages"]:
    for target in pkg["targets"]:
        if "bench" in target["kind"]:
            print(pkg["name"], target["name"])'
)
if [ "${#bench_targets[@]}" -eq 0 ]; then
  echo "bench: no bench targets found (add a [[bench]] target with harness = false)" >&2
  exit 1
fi

for target in "${bench_targets[@]}"; do
  package="${target%% *}"
  bench_name="${target##* }"
  echo "==> bench $package/$bench_name"
  cargo bench -p "$package" --bench "$bench_name" -- \
    --save-baseline current --noplot \
    --warm-up-time 0.5 --measurement-time 1.0 --sample-size 10 >/dev/null
done

python3 - <<'PYEMIT' >"$CURRENT"
import json
import pathlib

root = pathlib.Path("target/criterion")
rows = []
for estimates in sorted(root.glob("*/current/estimates.json")):
    meta = json.loads((estimates.parent / "benchmark.json").read_text())
    stats = json.loads(estimates.read_text())
    rows.append((meta["full_id"], float(stats["mean"]["point_estimate"]), "ns", "gate",
                 float(stats.get("std_dev", {}).get("point_estimate", 0.0))))
if not rows:
    raise SystemExit("bench: no criterion estimates under target/criterion/*/current")
for name, value, unit, mode, noise in sorted(rows):
    print(f"{name}\t{value:.3f}\t{unit}\t{mode}\t{noise:.3f}")
PYEMIT

python3 scripts/compare-bench.py "$BASELINE" "$CURRENT" \
  --threshold-pct "$THRESHOLD_PCT" "${UPDATE[@]+"${UPDATE[@]}"}"
