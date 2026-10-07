//! Integration test: spawns the real service binary on a scratch port and
//! probes liveness/readiness over raw HTTP (no extra deps). This keeps the
//! binary's `main` inside the coverage gate.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::time::{Duration, Instant};

fn http_get(port: u16, path: &str) -> Option<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok()?;
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
    )
    .ok()?;
    let mut buf = String::new();
    stream.read_to_string(&mut buf).ok()?;
    Some(buf)
}

fn spawn(port: u16) -> Option<Child> {
    Command::new(env!("CARGO_BIN_EXE_omni-service"))
        .env("OMNI_PORT", port.to_string())
        .spawn()
        .ok()
}

fn wait_listening(port: u16) -> bool {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    false
}

#[test]
fn health_and_readiness_answer() {
    let port: u16 = 18080 + u16::try_from(std::process::id() % 1000).unwrap_or(0);
    let mut child = spawn(port);
    assert!(child.is_some(), "service spawns");
    let Some(child) = child.as_mut() else { return };

    assert!(wait_listening(port), "service listens on {port}");
    let early_exit = child.try_wait();
    let still_running = early_exit.as_ref().is_ok_and(std::option::Option::is_none);
    assert!(still_running, "service exited early: {early_exit:?}");

    let health = http_get(port, "/healthz");
    assert!(health.is_some(), "healthz answered: {health:?}");
    let Some(health) = health else { return };
    assert!(health.contains("200 OK"), "healthz: {health}");
    assert!(health.contains("\"ok\""), "healthz: {health}");

    let ready = http_get(port, "/readyz");
    assert!(ready.is_some(), "readyz answered: {ready:?}");
    let Some(ready) = ready else { return };
    assert!(ready.contains("200 OK"), "readyz: {ready}");

    let killed = child.kill();
    assert!(killed.is_ok(), "service stops");
}
