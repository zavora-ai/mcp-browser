mod engine;
mod server;

use engine::BrowserEngine;
use rmcp::{ServiceExt, transport::stdio};
use server::BrowserServer;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
    let engine = BrowserEngine::launch()?;
    let service = BrowserServer { engine: Arc::new(Mutex::new(engine)) }.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
