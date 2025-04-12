# Data Model Design

## ClickHouse Schema

### 1. Transaction Records
```sql
CREATE TABLE transaction_records (
    block_number UInt64,
    transaction_hash FixedString(66),
    from_address FixedString(42),
    to_address FixedString(42),
    value Decimal128(18),
    timestamp DateTime,
    chain_id UInt64,
    status UInt8,
    
    INDEX idx_block (block_number) TYPE minmax,
    INDEX idx_addresses (from_address, to_address) TYPE minmax
)
ENGINE = ReplacingMergeTree()
ORDER BY (block_number, transaction_hash);
```

### 2. Balance Changes
```sql
CREATE TABLE balance_changes (
    address FixedString(42),
    block_number UInt64,
    change_amount Decimal128(18),
    transaction_hash FixedString(66),
    timestamp DateTime,
    chain_id UInt64,
    
    INDEX idx_address_block (address, block_number) TYPE minmax
)
ENGINE = ReplacingMergeTree()
ORDER BY (address, block_number);
```

### 3. Current Balances
```sql
CREATE MATERIALIZED VIEW current_balances
ENGINE = AggregatingMergeTree()
ORDER BY address AS
SELECT 
    address,
    chain_id,
    sum(change_amount) as balance,
    max(block_number) as last_block,
    max(timestamp) as last_update
FROM balance_changes
GROUP BY address, chain_id;
```

## Hypersync Data Mapping
```rust
// Transaction mapping
struct TransactionMapping {
    block_number: BlockNumber,
    transaction_hash: Hash,
    from_address: Address,
    to_address: Option<Address>,
    value: Quantity,
    timestamp: DateTime,
    chain_id: Quantity
}

// Balance change mapping
struct BalanceChangeMapping {
    address: Address,
    block_number: BlockNumber,
    change_amount: Quantity,
    transaction_hash: Hash,
    timestamp: DateTime
}
```
```

### 4. `PROCESSING_DESIGN.md`
```markdown
# Processing System Design

## Stream Processing

### 1. Hypersync Stream Configuration
```rust
StreamConfig {
    // Process in chunks of 1000 blocks
    batch_size: 1000,
    // Run 4 concurrent queries
    concurrency: 4,
    // Optimize field selection
    field_selection: vec![
        "block_number",
        "transaction_hash",
        "from",
        "to",
        "value"
    ]
}
```

### 2. Processing Pipeline
```rust
// Real-time processing
async fn process_stream() {
    let stream = client.stream(
        query,
        stream_config
    );
    
    while let Some(batch) = stream.next().await {
        process_batch(batch);
        update_state();
        handle_rollbacks();
    }
}

// Batch processing
async fn process_historical() {
    let response = client.collect(
        query,
        collect_config
    );
    
    process_response(response);
    update_state();
}
```

### 3. State Management
- Block processing state
- Balance calculation state
- Rollback handling
- Chain reorganization management

## Error Handling

### 1. Query Errors
- Retry logic
- Timeout handling
- Rate limiting
- Connection management

### 2. Processing Errors
- Data validation
- State recovery
- Inconsistency detection
- Error reporting
```

### 5. `API_DESIGN.md`
```markdown
# API Design

## Query Interface

### 1. Balance Queries
```graphql
type Balance {
    address: String!
    chainId: Int!
    balance: String!
    blockNumber: Int!
    timestamp: DateTime!
    lastTransactionHash: String
}

type Query {
    currentBalance(
        address: String!,
        chainId: Int!
    ): Balance

    balanceHistory(
        address: String!,
        chainId: Int!,
        fromBlock: Int!,
        toBlock: Int
    ): [Balance]
}
```

### 2. Real-time Updates
```graphql
type Subscription {
    balanceUpdates(
        address: String!,
        chainId: Int!
    ): Balance
}
```

### 3. Analytics Queries
```graphql
type Analytics {
    totalTransactions: Int!
    volumeStats: VolumeStats!
    timeSeriesData: [TimePoint!]!
}

type Query {
    getAnalytics(
        address: String!,
        chainId: Int!,
        timeRange: TimeRange!
    ): Analytics
}
```
```

Would you like me to elaborate on any specific aspect of these updated designs?