#![no_main]
//! Estate invariant: a parser either accepts or returns Err — never panics,
//! never aborts (THREAT-MODEL T1 / REQ-001).

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = omni_core::text::PubId::parse(s);
    }
});
