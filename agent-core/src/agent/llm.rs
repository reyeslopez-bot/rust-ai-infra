use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub async fn analyze_transaction(tx_data: &str) -> Result<String> {
    let prompt = format!(
        "Classify this transaction and output JSON with 'tag', 'summary', and 'risk_flag':\n{}",
        tx_data
    );

    let client = Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&OllamaRequest {
            model: "mistral", // Change to "llama3" or other if you prefer
            prompt: &prompt,
        })
        .send()
        .await
        .map_err(|e| anyhow!("❌ Failed to send request: {}", e))?;

    let text = res.text().await?;

    let last_response = text
        .lines()
        .filter_map(|line| serde_json::from_str::<OllamaResponse>(line).ok())
        .last()
        .ok_or_else(|| anyhow!("❌ No valid LLM response from Ollama"))?;

    Ok(last_response.response.trim().to_string())
}

