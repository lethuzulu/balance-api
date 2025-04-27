-- Transaction Records Table
CREATE TABLE transaction_records (
    block_number UInt64,
    transaction_hash String,
    from_address String,
    to_address String,
    value String, -- Using String for arbitrary precision values
    timestamp DateTime64(0),
    chain_id UInt64,
    is_success UInt8,
    
    INDEX idx_block (block_number) TYPE minmax,
    INDEX idx_addresses (from_address, to_address) TYPE set(100)
)
ENGINE = ReplacingMergeTree()
ORDER BY (chain_id, block_number, transaction_hash);

-- Balance Changes Table
CREATE TABLE balance_changes (
    address String,
    block_number UInt64,
    change_amount String, -- String for arbitrary precision
    transaction_hash String,
    timestamp DateTime64(0),
    chain_id UInt64,
    
    INDEX idx_address (address) TYPE set(100),
    INDEX idx_block (block_number) TYPE minmax
)
ENGINE = ReplacingMergeTree()
ORDER BY (chain_id, address, block_number);

-- Materialized View for Current Balances
CREATE MATERIALIZED VIEW current_balances
ENGINE = SummingMergeTree()
ORDER BY (chain_id, address) AS
SELECT 
    address,
    chain_id,
    sum(toDecimal128(change_amount, 18)) as balance,
    max(block_number) as last_block_number,
    max(timestamp) as last_update
FROM balance_changes
GROUP BY chain_id, address;

-- Historical Balances Table
CREATE TABLE historical_balances (
    address String,
    block_number UInt64,
    balance String,
    timestamp DateTime64(0),
    chain_id UInt64
)
ENGINE = ReplacingMergeTree()
ORDER BY (chain_id, address, block_number);