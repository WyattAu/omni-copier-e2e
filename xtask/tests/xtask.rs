//! Integration test: the xtask binary itself (unknown tasks exit 2).

use std::process::Command;

#[test]
fn unknown_task_exits_two() {
    let out = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("no-such-task")
        .output();
    assert!(out.is_ok(), "xtask runs: {out:?}");
    let Ok(out) = out else { return };
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown task"));
}
