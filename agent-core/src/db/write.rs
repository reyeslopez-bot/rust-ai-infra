use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use futures_util::stream::StreamExt;
use sqlx::PgPool;

pub async fn insert_tx_analysis(
    pool: &PgPool,
    tx_hash: &str,
    result: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query_unchecked!(
        "INSERT INTO tx_analysis (tx_hash, result) VALUES ($1, $2)",
        tx_hash,
        result
    )
    .execute(pool)
    .await?;

    Ok(())
}

