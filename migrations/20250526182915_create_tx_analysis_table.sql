CREATE TABLE tx_analysis (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR NOT NULL,
    result TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
