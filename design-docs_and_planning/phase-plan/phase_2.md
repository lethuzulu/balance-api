# Phase 2 Detailed Implementation Plan: Multi-Chain Support

## Overview

Phase 2 expands our ETH balance tracking system to support multiple EVM chains, including L2 networks, while maintaining the contract-address-only scope from Phase 1.

## Implementation by Layer

### 1. Data Collection Layer

#### Functions to Implement

```rust
// Initialize and configure Hypersync clients for multiple chains
fn initializeMultiChainClients(chainConfigs) -> Result<HashMap<u64, Client>>

// Optimize queries for specific chain characteristics
fn configureChainSpecificQueries(chainId: u64) -> Result<QueryConfig>

// Track cross-chain asset movements
fn monitorL2BridgeTransactions(l1Address: String, l2Address: String) -> Result<()>

// Chain-specific transaction retrieval
fn queryChainSpecificTransactions(chainId: u64, address: String, fromBlock: u64, toBlock: Option<u64>) -> Result<QueryResponse>

// Coordinate syncing across multiple chains
fn scheduleSyncJobs(chainPriorities: HashMap<u64, Priority>) -> Result<()>
```

#### Tasks

- Create chain-specific configurations for each supported chain
- Implement adapters for different L2 transaction formats
- Build coordination mechanism for multiple Hypersync clients
- Set up multi-chain Kafka topics
- Develop chain-specific error handling

#### Schema Design

```rust
// Enhanced Kafka message schema
struct MultiChainTransactionEvent {
    chain_id: u64,
    chain_type: ChainType,  // Enum: L1, OptimisticRollup, ZKRollup, etc.
    transaction_hash: String,
    block_number: u64,
    from_address: String,
    to_address: String,
    value: BigDecimal,
    timestamp: DateTime,
    is_success: bool,
    l1_transaction_hash: Option<String>, // For L2 transactions with L1 settlement
    l1_fee: Option<BigDecimal>,          // L2-specific fees
}

enum ChainType {
    L1,
    OptimisticRollup,
    ZKRollup,
    Sidechain,
}
```

#### Components

- Multiple Hypersync Clients (one per chain)
- Chain-specific Kafka Producers
- Redis (for multi-chain sync state)

### 2. Storage Layer

#### Functions to Implement

```rust
// Set up database structures for multiple chains
fn createMultiChainSchema() -> Result<()>

// Store chain-specific transaction data
fn storeChainSpecificTransaction(chainId: u64, transaction: MultiChainTransactionEvent) -> Result<()>

// Link related cross-chain transactions
fn storeCrossChainTransaction(sourceTx: String, destTx: String) -> Result<()>

// Aggregate balances across multiple chains
fn getBalanceAcrossChains(address: String) -> Result<HashMap<u64, BigDecimal>>

// Chain-specific balance retrieval
fn getChainSpecificBalance(chainId: u64, address: String, blockNumber: u64) -> Result<BigDecimal>
```

#### Tasks

- Extend database schema for multi-chain support
- Implement chain-specific partitioning
- Create cross-chain aggregation views
- Set up chain-specific caching strategies
- Implement data consistency rules across chains

#### Schema Design

```sql
-- ClickHouse schema extensions
CREATE TABLE chains (
    chain_id UInt64,
    chain_name String,
    chain_type Enum8('L1' = 1, 'OptimisticRollup' = 2, 'ZKRollup' = 3, 'Sidechain' = 4),
    parent_chain_id Nullable(UInt64),
    is_active UInt8
) ENGINE = ReplacingMergeTree()
ORDER BY chain_id;

-- Updated tables with chain_id partition
CREATE TABLE transactions (
    chain_id UInt64,
    transaction_hash String,
    block_number UInt64,
    from_address String,
    to_address String,
    value Decimal128(18),
    timestamp DateTime,
    status UInt8,
    l1_transaction_hash Nullable(String),
    l1_fee Nullable(Decimal128(18))
) ENGINE = ReplacingMergeTree()
PARTITION BY chain_id
ORDER BY (transaction_hash, chain_id);

-- Cross-chain transactions
CREATE TABLE cross_chain_transactions (
    source_chain_id UInt64,
    source_transaction_hash String,
    destination_chain_id UInt64,
    destination_transaction_hash String,
    address String,
    value Decimal128(18),
    timestamp DateTime
) ENGINE = ReplacingMergeTree()
ORDER BY (source_chain_id, source_transaction_hash);

-- Multi-chain view
CREATE MATERIALIZED VIEW aggregated_balances
ENGINE = SummingMergeTree()
ORDER BY (address) AS
SELECT 
    address,
    chain_id,
    sum(final_balance) as chain_balance
FROM 
    running_balances
GROUP BY address, chain_id;
```

#### Components

- ClickHouse (with multi-chain partitioning)
- Redis Cache (with chain-specific keys)

### 3. Processing Layer

#### Functions to Implement

```rust
// Chain-specific transaction handling
fn processChainSpecificTransaction(chainId: u64, transaction: MultiChainTransactionEvent) -> Result<()>

// Identify L1-L2 bridge transactions
fn detectCrossChainMovements(transaction: MultiChainTransactionEvent) -> Result<Option<CrossChainInfo>>

// Ensure consistent cross-chain balances
fn syncBalancesAcrossChains(address: String) -> Result<()>

// Chain-specific reorganization handling
fn handleChainReorgs(chainId: u64, fromBlock: u64) -> Result<()>

// Process L2-specific fee structures
fn calculateL2SpecificFees(transaction: MultiChainTransactionEvent) -> Result<BigDecimal>
```

#### Tasks

- Create chain-specific processing pipelines
- Implement cross-chain transaction linking
- Build L2-specific fee calculations
- Develop chain-specific reorg handling
- Create multi-chain validation rules

#### Components

- Chain-specific Kafka Consumers
- Enhanced Kafka Streams
- Redis (for cross-chain state)

### 4. Query Layer

#### Functions to Implement

```rust
// Get aggregated balance across chains
fn getMultiChainBalance(address: String) -> Result<HashMap<u64, BigDecimal>>

// Get chain-specific balance
fn getChainSpecificBalance(chainId: u64, address: String, blockNumber: u64) -> Result<BigDecimal>

// Get transactions across all chains
fn getMultiChainTransactionHistory(address: String) -> Result<Vec<MultiChainTransactionEvent>>

// Get transactions that move assets between chains
fn getCrossChainTransactions(address: String) -> Result<Vec<CrossChainTransaction>>

// Get historical balances across chains
fn getConsolidatedBalanceHistory(address: String, timeRange: TimeRange) -> Result<Vec<BalanceSnapshot>>
```

#### Tasks

- Enhance API services for multi-chain queries
- Implement cross-chain aggregation resolvers
- Create chain-specific query optimization
- Develop multi-chain authorization rules
- Build chain status health endpoints

#### Schema Design

```protobuf
// Enhanced gRPC service definition
service MultiChainBalanceService {
  rpc GetMultiChainBalance(AddressRequest) returns (MultiChainBalanceResponse);
  rpc GetChainSpecificBalance(ChainBalanceRequest) returns (BalanceResponse);
  rpc GetCrossChainTransactions(AddressRequest) returns (CrossChainTransactionsResponse);
  rpc GetConsolidatedBalanceHistory(TimeRangeRequest) returns (ConsolidatedBalanceHistoryResponse);
}

// Enhanced GraphQL schema
type Query {
  multiChainBalance(address: String!): MultiChainBalance
  chainSpecificBalance(address: String!, chainId: Int!, blockNumber: Int): Balance
  crossChainTransactions(address: String!, limit: Int): [CrossChainTransaction]
  consolidatedBalanceHistory(address: String!, fromTimestamp: DateTime, toTimestamp: DateTime): [BalanceSnapshot]
}
```

#### Components

- Enhanced gRPC Server
- Enhanced GraphQL Server

## Cross-Layer Tasks

- Develop configuration management for multiple chains
- Implement multi-chain monitoring
- Create chain prioritization logic
- Build cross-chain data consistency validation
- Implement chain-specific error handling

## Success Criteria

### Functionality Criteria

- System tracks ETH balances for contract addresses across all configured chains
- Cross-chain transactions are properly linked and accounted for
- L2-specific fee structures are correctly calculated
- Aggregated balances across chains are accurate
- Real-time updates work for all supported chains

### Performance Criteria

- Historical sync processes at least 100,000 blocks per hour per chain
- Query response time < 250ms for current balance across all chains
- Query response time < 600ms for historical balance across all chains
- System handles at least 40 concurrent queries per chain

### Reliability Criteria

- No data loss during chain-specific reorganizations
- 99.9% uptime for query services across all chains
- Automatic recovery from chain-specific disconnections
- Consistent balances across query methods and chains

### Validation Tests

- Test against known contract addresses on multiple chains
- Verify L2 fee calculations against chain documentation
- Test cross-chain transactions through major bridges
- Verify multi-chain balance consistency
- Successfully process transactions during network upgrades
- Test system resilience to chain-specific issues
