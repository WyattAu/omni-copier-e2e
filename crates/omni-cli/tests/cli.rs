//! Integration test: spawns the real binary (`CARGO_BIN_EXE_*`) so `main`
//! itself is exercised — the coverage gate counts it.

use std::process::Command;

#[test]
fn prints_valid() {
    let out = Command::new(env!("CARGO_BIN_EXE_omni-cli"))
        .arg("hello-world")
        .output();
    assert!(out.is_ok(), "binary runs: {out:?}");
    let Ok(out) = out else { return };
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout.trim(), "valid: hello-world");
}

#[test]
fn rejects_invalid() {
    let out = Command::new(env!("CARGO_BIN_EXE_omni-cli"))
        .arg("bad\nid")
        .output();
    assert!(out.is_ok(), "binary runs: {out:?}");
    let Ok(out) = out else { return };
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("error:"));
}
