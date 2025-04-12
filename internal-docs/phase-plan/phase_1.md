# Phase 1: Ethereum Mainnet Basic Implementation

## Overview

Phase 1 focuses on building the foundation of our ETH balance tracking system for contract addresses on Ethereum Mainnet, without handling edge cases.

## Implementation by Layer

### 1. Data Collection Layer

#### Functions to Implement

```rust
// Initialize and configure Hypersync client
fn initializeHypersyncClient() -> Result<Client>

// Query incoming transactions to an address
fn queryTransactionsToAddress(address: String, fromBlock: u64, toBlock: Option<u64>) -> Result<QueryResponse>

// Query outgoing transactions from an address
fn queryTransactionsFromAddress(address: String, fromBlock: u64, toBlock: Option<u64>) -> Result<QueryResponse>

// Handle pagination from Hypersync responses
fn handlePagination(queryResponse: QueryResponse) -> Result<u64>

// Set up real-time transaction monitoring
fn streamLatestTransactions(address: String) -> Result<Receiver<Result<QueryResponse>>>
```

#### Tasks

- Set up Hypersync client with proper configuration
- Create custom query builders with field selection optimization
- Implement pagination handling for historical syncing
- Build Kafka producer for transaction events
- Implement health check and reconnection logic

#### Schema Design

```rust
// Kafka message schema
struct TransactionEvent {
    chain_id: u64,
    transaction_hash: String,
    block_number: u64,
    from_address: String,
    to_address: String,
    value: BigDecimal,
    timestamp: DateTime,
    is_success: bool,
}
```

#### Components

- Hypersync Client
- Kafka Producer
- Redis (for sync state)

### 2. Storage Layer

#### Functions to Implement

```rust
// Initialize database tables and indexes
fn createSchemaStructures() -> Result<()>

// Save transaction data
fn storeTransaction(transaction: TransactionEvent) -> Result<()>

// Record balance changes
fn storeBalanceChange(address: String, blockNumber: u64, change: BigDecimal) -> Result<()>

// Retrieve current balance
fn getLatestBalance(address: String) -> Result<BigDecimal>

// Get historical balance
fn getBalanceAtBlock(address: String, blockNumber: u64) -> Result<BigDecimal>
```

#### Tasks

- Design and create ClickHouse tables and indexes
- Implement efficient insert operations for transactions
- Create materialized views for running balances
- Set up Redis caching for frequent balance queries
- Configure data retention policies

#### Schema Design

```sql
-- ClickHouse schema
CREATE TABLE transactions (
    chain_id UInt64,
    transaction_hash String,
    block_number UInt64,
    from_address String,
    to_address String,
    value Decimal128(18),
    timestamp DateTime,
    status UInt8
) ENGINE = ReplacingMergeTree()
ORDER BY (transaction_hash, chain_id);

CREATE TABLE balance_changes (
    address String,
    block_number UInt64,
    change_amount Decimal128(18),
    transaction_hash String,
    timestamp DateTime
) ENGINE = ReplacingMergeTree()
ORDER BY (address, block_number);

-- Materialized view for running balances
CREATE MATERIALIZED VIEW running_balances
ENGINE = AggregatingMergeTree()
ORDER BY (address, block_number) AS
SELECT 
    address,
    block_number,
    sum(change_amount) OVER (
        PARTITION BY address 
        ORDER BY block_number
    ) as running_balance
FROM balance_changes;
```

#### Components

- ClickHouse
- Redis Cache

### 3. Processing Layer

#### Functions to Implement

```rust
// Process raw transaction data
fn processTransaction(transaction: TransactionEvent) -> Result<()>

// Determine balance impact
fn calculateBalanceChange(transaction: TransactionEvent) -> Result<BigDecimal>

// Update running balance
fn updateRunningBalance(address: String, blockNumber: u64, change: BigDecimal) -> Result<()>

// Handle chain reorganizations
fn handleReorg(fromBlock: u64) -> Result<()>

// Create balance checkpoints
fn createCheckpoints(address: String, blockNumber: u64) -> Result<()>
```

#### Tasks

- Implement Kafka consumers for transaction events
- Build balance calculation logic
- Create checkpoint management system
- Implement basic validation rules
- Set up state tracking for sync progress

#### Components

- Kafka Consumers
- Kafka Streams
- Redis (for state)

### 4. Query Layer

#### Functions to Implement

```rust
// Get latest balance
fn getCurrentBalance(address: String) -> Result<BigDecimal>

// Get balance at block
fn getHistoricalBalance(address: String, blockNumber: u64) -> Result<BigDecimal>

// List transactions
fn getTransactionHistory(address: String, fromBlock: u64, toBlock: u64) -> Result<Vec<Transaction>>

// Show balance changes
fn getBalanceHistory(address: String, fromBlock: u64, toBlock: u64) -> Result<Vec<BalanceChange>>

// System health status
fn healthCheck() -> Result<SystemStatus>
```

#### Tasks

- Design and implement gRPC service definitions
- Create GraphQL schema and resolvers
- Implement query optimizations
- Set up basic authorization
- Create health check endpoints

#### Schema Design

```protobuf
// gRPC service definition
service BalanceService {
  rpc GetCurrentBalance(AddressRequest) returns (BalanceResponse);
  rpc GetHistoricalBalance(HistoricalRequest) returns (BalanceResponse);
  rpc GetTransactionHistory(TransactionHistoryRequest) returns (TransactionHistoryResponse);
  rpc GetBalanceHistory(BalanceHistoryRequest) returns (BalanceHistoryResponse);
}

// GraphQL schema
type Query {
  currentBalance(address: String!): Balance
  historicalBalance(address: String!, blockNumber: Int!): Balance
  transactionHistory(address: String!, fromBlock: Int, toBlock: Int, limit: Int): [Transaction]
  balanceHistory(address: String!, fromBlock: Int, toBlock: Int): [BalanceChange]
}
```

#### Components

- gRPC Server
- GraphQL Server

## Cross-Layer Tasks

- Set up Kubernetes configurations for all services
- Configure monitoring and logging
- Implement error handling and retry logic
- Create system health checks
- Develop integration tests

## Success Criteria

### Functionality Criteria

- System can track ETH balance changes for any contract address
- Historical balance queries return correct values at any block number
- Real-time balance updates are reflected within 30 seconds of chain confirmation
- Transaction history is complete and accurate

### Performance Criteria

- Historical sync processes at least 100,000 blocks per hour
- Query response time < 200ms for current balance
- Query response time < 500ms for historical balance
- System handles at least 50 concurrent queries

### Reliability Criteria

- No data loss during chain reorganizations
- 99.9% uptime for query services
- Automatic recovery from Hypersync or database disconnections
- Consistent balances across query methods

### Validation Tests

- Test against 10+ known contract addresses with verified balances
- Verify system against etherscan or similar block explorer balances
- Successfully process high-volume contracts (>10,000 transactions)
- Verify balance consistency during network congestion
