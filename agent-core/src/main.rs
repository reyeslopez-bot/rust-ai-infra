use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use tokio::time::{sleep, Duration};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL")
        .expect("Missing DATABASE_URL env var");
    let pool = PgPool::connect(&db_url).await?;

    info!("🚀 Agent started and connected to the DB.");

    loop {
        match do_agent_task(&pool).await {
            Ok(_) => info!("✅ Agent task complete."),
            Err(e) => error!("❌ Agent task failed: {e}"),
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn do_agent_task(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 🧠 Replace this with actual logic (e.g., fetch tasks, write logs)
    let result = sqlx::query!("SELECT now() as time").fetch_one(pool).await?;
    info!("⏱️ DB Time: {:?}", result.time);
    Ok(())
}

