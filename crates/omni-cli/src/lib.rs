//! Library wrapper: keeps the binary a thin shell so the logic is
//! unit-testable (and coverage counts it).

/// Parse CLI arguments and produce the output line.
///
/// # Errors
///
/// Returns the message to print on stderr when the id is invalid.
pub fn run(id: &str) -> Result<String, String> {
    match omni_core::text::PubId::parse(id) {
        Ok(parsed) => Ok(format!("valid: {parsed}")),
        Err(e) => Err(format!("error: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn valid_id() {
        assert_eq!(run(" hello "), Ok("valid: hello".to_owned()));
    }

    #[test]
    fn invalid_id() {
        assert!(run("a\nb").is_err());
    }
}
