//! Criterion bench — wired to the shared CI's bench-regression input
//! (latency-relevant crates keep committed budgets; see percentile-kit).
//!
//! The `criterion` macros generate undocumented items, so `missing_docs` is
//! waived for this file only (estate lints stay strict everywhere else).
#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use omni_core::text::PubId;

/// Bench the parser — wired to the shared CI's bench-regression input
/// (latency-relevant crates keep committed budgets; see percentile-kit).
fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse valid id", |b| {
        b.iter(|| PubId::parse("  omni-core "));
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
