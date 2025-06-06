mod agent;
mod db;
use agent::processor::run_agent;
use agent::db::init::create_pool;
use tracing::{info};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from `.env`
    dotenv().ok();

    // Initialize logging (can be extended to JSON/logfmt/file output)
    tracing_subscriber::fmt::init();

    // Create a connection pool to CockroachDB / PostgreSQL
    let pool = create_pool().await?;
    info!("🚀 Agent started and connected to the DB.");

    run_agent(pool).await;
    Ok(())

}

