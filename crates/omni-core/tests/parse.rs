//! Property tests (proptest — estate parity with the Rust kits' property
//! class): parsing never panics on arbitrary input (REQ-001).

use omni_core::text::PubId;
use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_never_panics(input in ".{0,600}") {
        let _ = PubId::parse(&input);
    }

    #[test]
    fn valid_input_round_trips(trimmed in "[!-~]{1,256}") {
        match PubId::parse(&trimmed) {
            Ok(id) => prop_assert_eq!(id.as_str(), trimmed),
            Err(e) => prop_assert!(false, "generator produced valid input but parse failed: {e}"),
        }
    }
}
