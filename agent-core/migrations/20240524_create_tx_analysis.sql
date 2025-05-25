-- migrations/20240524_create_tx_analysis.sql
CREATE TABLE IF NOT EXISTS tx_analysis (
    tx_hash TEXT PRIMARY KEY,
    result TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

