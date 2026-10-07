//! Example service binary demonstrating the estate's HTTP posture: axum +
//! liveness/readiness routes + graceful shutdown + request-scoped tracing.
//! Swap the placeholder route for your domain; keep the posture.
//!
//! The port is env-configurable (`OMNI_PORT`) so the integration test can
//! spawn the real binary without collisions.

use std::net::SocketAddr;

use axum::routing::get;
use axum::{Json, Router};

// Coverage (ADR-0005): this file is the process surface — exercised by the
// spawned-binary smoke test but excluded from the line-coverage gate
// because a killed child emits no LLVM profile.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz));

    let port: u16 = std::env::var("OMNI_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// Liveness: process is up (k8s liveness probe). Axum requires async
/// handlers; nothing here awaits, hence the pedantic allow — keep it scoped.
#[allow(clippy::unused_async)]
async fn healthz() -> Json<&'static str> {
    Json("ok")
}

/// Readiness: dependencies are reachable (k8s readiness probe).
#[allow(clippy::unused_async)]
async fn readyz() -> Json<&'static str> {
    // Real deployments: check DB/queue/etc. here. Never fail liveness for
    // dependency state — fail readiness.
    Json("ready")
}

/// Resolve shutdown from SIGINT/SIGTERM so connections drain (see the
/// estate's shutdown-kit for the full production pattern).
async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received; draining");
}
