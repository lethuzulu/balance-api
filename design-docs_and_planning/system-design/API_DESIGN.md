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
