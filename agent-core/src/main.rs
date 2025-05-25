mod agent;
mod db;

use crate::agent::processor::run_agent;
use db::init::init_pool;

use tracing::{info, error};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from `.env`
    dotenv().ok();

    // Initialize logging (can be extended to JSON/logfmt/file output)
    tracing_subscriber::fmt::init();

    // Create a connection pool to CockroachDB / PostgreSQL
    let pool = init_pool().await?;
    info!("🚀 Agent started and connected to the DB.");

    // Start agent logic and log if it fails
    if let Err(e) = run_agent(&pool).await {
        error!("❌ Agent crashed: {:?}", e);
    }

    Ok(())
}

