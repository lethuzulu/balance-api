# API Specification

## 1. gRPC Services

### 1.1 Balance Service
```protobuf
service BalanceService {
    // Get current balance
    rpc GetBalance (BalanceRequest) returns (BalanceResponse);
    
    // Stream balance updates
    rpc StreamBalanceUpdates (BalanceRequest) returns (stream BalanceUpdate);
    
    // Get historical balances
    rpc GetHistoricalBalances (HistoricalRequest) returns (HistoricalResponse);
}

message BalanceRequest {
    string address = 1;
    uint64 chain_id = 2;
}
```

### 1.2 Transaction Service
```protobuf
service TransactionService {
    rpc GetTransactions (TransactionRequest) returns (TransactionResponse);
    rpc StreamTransactions (TransactionRequest) returns (stream Transaction);
}
```

## 2. GraphQL Schema

### 2.1 Types
```graphql
type Balance {
    address: String!
    chainId: Int!
    balance: String!
    blockNumber: Int!
    timestamp: DateTime!
}

type Transaction {
    hash: String!
    from: String!
    to: String!
    value: String!
    blockNumber: Int!
}
```

### 2.2 Queries and Subscriptions
```graphql
type Query {
    currentBalance(address: String!, chainId: Int!): Balance
    balanceHistory(
        address: String!, 
        chainId: Int!,
        fromBlock: Int!,
        toBlock: Int
    ): [Balance]
}

type Subscription {
    balanceUpdates(address: String!, chainId: Int!): Balance
}
```
```

4. `DATA_SCHEMA.md`:
```markdown
# Data Schema Design

## 1. ClickHouse Schema

### 1.1 Core Tables

#### Chains
```sql
CREATE TABLE chains (
    chain_id UInt64,
    chain_type Enum8('L1' = 1, 'L2' = 2),
    chain_config String,
    created_at DateTime,
    updated_at DateTime
) ENGINE = ReplacingMergeTree()
ORDER BY chain_id;
```

#### Addresses
```sql
CREATE TABLE addresses (
    address FixedString(42),
    address_type Enum8('Contract' = 1, 'EOA' = 2),
    deployment_block UInt64,
    deployment_timestamp DateTime,
    last_activity_block UInt64,
    last_activity_timestamp DateTime
) ENGINE = ReplacingMergeTree()
ORDER BY address;
```

### 1.2 Transaction Tables

#### Balance Changes
```sql
CREATE TABLE balance_changes (
    address FixedString(42),
    chain_id UInt64,
    block_number UInt64,
    transaction_hash FixedString(66),
    change_amount Decimal128(18),
    timestamp DateTime,
    change_type Enum8('In' = 1, 'Out' = 2)
) ENGINE = ReplacingMergeTree()
ORDER BY (address, chain_id, block_number);
```

## 2. Redis Schema

### 2.1 Cache Keys
```
# Current Balance
balance:{address}:{chain_id} -> {
    "balance": "100000000000000000",
    "block_number": 15000000,
    "timestamp": 1678234567
}

# Processing State
processing:{chain_id}:{block_number} -> {
    "status": "processing",
    "start_time": 1678234567,
    "transaction_count": 150
}
