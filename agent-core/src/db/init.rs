use sqlx::PgPool;
use std::env;

pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL")
        .expect("Missing DATABASE_URL env var");
    PgPool::connect(&db_url).await
}

