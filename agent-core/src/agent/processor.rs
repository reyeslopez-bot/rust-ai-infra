// agent-core/src/agent/processor.rs
use sqlx::PgPool;
use rdkafka::{consumer::StreamConsumer, ClientConfig, Message};
use tracing::{info, error};
use crate::agent::llm::analyze_transaction;
use crate::db::write::insert_tx_analysis;

pub async fn run_agent(pool: &PgPool) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "wallet-agent")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Kafka consumer creation failed");

    consumer.subscribe(&["wallet_transactions"]).expect("Subscription failed");

    let mut stream = consumer.stream();

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(m) => {
                if let Some(payload) = m.payload_view::<str>().ok().flatten() {
                    match analyze_transaction(payload).await {
                        Ok(result) => {
                            let tx_hash = "some_hash"; // TODO: extract from `payload`
                            if let Err(e) = insert_tx_analysis(pool, tx_hash, &result).await {
                                error!("Failed to insert tx: {e}");
                            } else {
                                info!("Saved tx analysis");
                            }
                        }
                        Err(e) => error!("LLM error: {e}"),
                    }
                }
            }
            Err(e) => error!("Kafka error: {e}"),
        }
    }
}

