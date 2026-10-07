#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
//! Leaf primitives for the `OmniRust` workspace.
//!
//! This crate is the L0 pattern made concrete: zero runtime workspace
//! dependencies, `no_std`-capable via the default-on `std` feature, and
//! requirement-tagged tests (see REQUIREMENTS.md at the repo root).

extern crate alloc;

pub mod text;
