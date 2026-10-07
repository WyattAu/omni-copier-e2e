# 0003 — no_std policy for leaf crates

Date: 2026-10-04

## Status

Accepted

## Context

The estate's embedded/no_std audit (engineering-standards README) settled a
house pattern: `std` is a default-on feature gating whole items, never a
silent behavioral downgrade.

## Decision

- Leaves start with `#![cfg_attr(not(feature = "std"), no_std)]` +
  `#![forbid(unsafe_code)]`.
- Anything needing the OS (wall clock, filesystem, threads) is gated behind
  `feature = "std"`.
- No `alloc` requirement until the crate actually needs it; when it does,
  `extern crate alloc` + hashbrown with default hasher (no_std HashMap).
- The shared CI's `embedded: true` input adds the `thumbv7em-none-eabihf`
  check when a crate claims no_std.

## Consequences

- Higher discipline cost up front; leaves stay portable to embedded and wasm
  without a rewrite later.
