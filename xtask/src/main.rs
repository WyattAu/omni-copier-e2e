//! `cargo xtask <task>` — the workspace task runner. The Makefile and CI
//! both call these; there is exactly one definition of every gate
//! (Omni Core Contract: scripts canonical, no CI-only steps).

use std::process::Command;

fn main() {
    let task = std::env::args().nth(1).unwrap_or_else(|| "help".into());
    match task.as_str() {
        "coverage" => run(
            "cargo",
            &[
                "llvm-cov",
                "--all-features",
                "--summary-only",
                "--fail-under",
                "90",
            ],
        ),
        "mutants" => run("cargo", &["mutants", "--all-features", "-o", "mutants-out"]),
        "vet" => run("cargo", &["vet", "check", "--all-features"]),
        "semver" => run("cargo", &["semver-checks", "check", "--workspace"]),
        "contract" => sh("scripts/check-contract.sh"),
        other => {
            eprintln!(
                "unknown task: {other}\nusage: cargo xtask <coverage|mutants|vet|semver|contract>"
            );
            std::process::exit(2);
        }
    }
}

fn run(bin: &str, args: &[&str]) {
    let status = match Command::new(bin).args(args).status() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("error: cannot spawn {bin}: {e}");
            std::process::exit(2);
        }
    };
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

// Coverage: `sh` delegates to `run` (see above).
fn sh(script: &str) {
    run("bash", &[script]);
}
