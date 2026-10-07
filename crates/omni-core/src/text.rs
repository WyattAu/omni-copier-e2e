//! Checked text primitives. REQ-001: parsing rejects invalid input with
//! `Err`, never panics; REQ-101 (security): parsing bounds memory use via a
//! hard length cap.

use alloc::string::String;

/// Hard upper bound — parsing must not allocate unbounded strings (REQ-101).
pub const MAX_LEN: usize = 256;

/// Error type for [`PubId::parse`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    /// Input exceeded [`MAX_LEN`].
    #[error("input exceeds {MAX_LEN} bytes")]
    TooLong,
    /// Input was empty or whitespace-only.
    #[error("input is empty")]
    Empty,
    /// Input contained a byte outside the allowed alphabet.
    #[error("invalid byte {0:#04x}")]
    InvalidByte(u8),
}

/// A validated public identifier: 1..=`MAX_LEN` bytes, ASCII graphic, trimmed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PubId(String);

impl PubId {
    /// Parse and validate.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError::Empty`] on empty/whitespace input,
    /// [`ParseError::TooLong`] beyond [`MAX_LEN`] bytes, and
    /// [`ParseError::InvalidByte`] for bytes outside printable ASCII
    /// (REQ-001: rejects with `Err`, never panics).
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(ParseError::Empty);
        }
        if trimmed.len() > MAX_LEN {
            return Err(ParseError::TooLong);
        }
        for &b in trimmed.as_bytes() {
            if !(0x21..=0x7e).contains(&b) {
                return Err(ParseError::InvalidByte(b));
            }
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Borrow the validated identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for PubId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid() -> Result<(), ParseError> {
        let id = PubId::parse("  hello-world ")?;
        assert_eq!(id.as_str(), "hello-world");
        Ok(())
    }

    #[test]
    fn rejects_empty() {
        assert!(matches!(PubId::parse("   "), Err(ParseError::Empty)));
    }

    #[test]
    fn rejects_too_long() {
        assert!(matches!(
            PubId::parse(&"x".repeat(MAX_LEN + 1)),
            Err(ParseError::TooLong)
        ));
    }

    #[test]
    fn rejects_control_bytes() {
        assert!(matches!(
            PubId::parse("a\nb"),
            Err(ParseError::InvalidByte(_))
        ));
    }
}
