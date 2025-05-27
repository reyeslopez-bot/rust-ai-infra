use std::error::Error;
use std::time::Duration;

use futures_util::StreamExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::ClientConfig;
use serde::Deserialize;
use sqlx::PgPool;
use tokio::time::sleep;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
struct TxPayload {
    tx_hash: String,
    // Add other relevant fields here
}

async fn insert_tx_analysis(
    pool: &PgPool,
    tx_hash: &str,
    result: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO tx_analysis (tx_hash, result)
        VALUES ($1, $2)
        "#,
        tx_hash,
        result
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn run_agent(pool: PgPool) -> Result<(), Box<dyn Error>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "wallet-analyzer")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()?;

    consumer.subscribe(&["wallet_transactions"])?;

    let mut stream = consumer.stream();

    while let Some(msg_result) = stream.next().await {
        match msg_result {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    match std::str::from_utf8(payload) {
                        Ok(payload_str) => {
                            match serde_json::from_str::<TxPayload>(payload_str) {
                                Ok(tx) => {
                                    // Simulate processing and generating a result
                                    let result = format!("Analysis result for {}", tx.tx_hash);

                                    match insert_tx_analysis(&pool, &tx.tx_hash, &result).await {
                                        Ok(_) => info!("Saved tx analysis for {}", tx.tx_hash),
                                        Err(e) => {
                                            error!("Failed to insert tx: {}", e);
                                            if let Some(source) = e.source() {
                                                error!("Caused by: {}", source);
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to deserialize payload: {}. Payload: {}", e, payload_str);
                                }
                            }
                        }
                        Err(e) => error!("Payload is not valid UTF-8: {}", e),
                    }
                } else {
                    error!("Kafka message had no payload");
                }
            }
            Err(e) => error!("Kafka error: {}", e),
        }

        // Optional: Sleep to prevent tight loop in case of continuous errors
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
