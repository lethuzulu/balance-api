
# File and Folder Structure for Phase 1

## Root Directory Structure

```
balances_api/
├── src/
│   ├── data_collection/
│   ├── storage/
│   ├── processing/
│   ├── query/
│   ├── common/
│   └── main.rs
├── config/
├── tests/
├── Cargo.toml
├── docker-compose.yml
└── README.md

```

## Detailed Component Structure

### Data Collection Layer

```
src/data_collection/
├── mod.rs                         # Module exports
├── hypersync/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # HyperSync client wrapper
│   ├── transaction_collector.rs   # Collects transactions
│   └── config.rs                  # Configuration for HyperSync
├── models/
│   ├── mod.rs                     # Module exports
│   ├── transaction.rs             # Transaction data model
│   └── block.rs                   # Block data model
├── kafka/
│   ├── mod.rs                     # Module exports
│   └── producer.rs                # Kafka producer for transaction events
└── service.rs                     # Data collection service

```

### Storage Layer

```
src/storage/
├── mod.rs                         # Module exports
├── clickhouse/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # ClickHouse client
│   ├── schema.rs                  # Database schema definitions
│   └── migrations/                # Database migrations
│       ├── mod.rs
│       └── v1_initial_setup.rs    # Initial schema setup
├── redis/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # Redis client
│   └── cache.rs                   # Caching implementation
├── models/
│   ├── mod.rs                     # Module exports
│   ├── transaction_record.rs      # Transaction record model
│   └── balance_record.rs          # Balance record model
└── repositories/
    ├── mod.rs                     # Module exports
    ├── transaction_repository.rs  # Transaction data repository
    └── balance_repository.rs      # Balance data repository

```

### Processing Layer

```
src/processing/
├── mod.rs                         # Module exports
├── kafka/
│   ├── mod.rs                     # Module exports
│   └── consumer.rs                # Kafka consumer for transaction events
├── processors/
│   ├── mod.rs                     # Module exports
│   ├── transaction_processor.rs   # Process transactions
│   └── balance_processor.rs       # Update balances based on transactions
├── models/
│   ├── mod.rs                     # Module exports
│   └── balance_change.rs          # Balance change model
└── service.rs                     # Processing service

```

### Query Layer

```
src/query/
├── mod.rs                         # Module exports
├── grpc/
│   ├── mod.rs                     # Module exports
│   ├── server.rs                  # gRPC server implementation
│   ├── balance_service.rs         # Balance service implementation
│   └── proto/                     # Protocol buffer definitions
│       ├── balance.proto          # Balance service definition
│       └── generated/             # Generated code from protobuf
├── graphql/
│   ├── mod.rs                     # Module exports
│   ├── schema.rs                  # GraphQL schema
│   ├── resolvers/                 # GraphQL resolvers
│   │   ├── mod.rs
│   │   └── balance_resolver.rs    # Balance resolver
│   └── server.rs                  # GraphQL server
└── service.rs                     # Query service

```

### Common Code

```
src/common/
├── mod.rs                         # Module exports
├── types.rs                       # Common type definitions
├── utils/
│   ├── mod.rs                     # Module exports
│   ├── ethereum.rs                # Ethereum-specific utilities
│   └── decimal.rs                 # Decimal handling utilities
├── error.rs                       # Error handling
└── config.rs                      # Configuration management

```

### Configuration Files

```
config/
├── default.toml                   # Default configuration
├── development.toml               # Development environment config
├── production.toml                # Production environment config
└── test.toml                      # Test environment config

```

### Test Directory

```
tests/
├── integration/                   # Integration tests
│   ├── data_collection_tests.rs   # Data collection tests
│   ├── storage_tests.rs           # Storage layer tests
│   ├── processing_tests.rs        # Processing layer tests
│   └── query_tests.rs             # Query layer tests
└── e2e/                           # End-to-end tests
    └── balance_tracking_test.rs   # Balance tracking E2E test

```

## Key Components to Implement in Phase 1

### Data Collection Layer

- HyperSync client for fetching transactions and blocks
- Transaction collector to gather transaction data
- Kafka producer for publishing transaction events

### Storage Layer

- ClickHouse database schema and client
- Redis caching system for frequently accessed balances
- Transaction and balance repositories

### Processing Layer

- Kafka consumer to process transaction events
- Transaction processor to analyze transactions
- Balance processor to update address balances

### Query Layer

- gRPC server and balance service implementation
- GraphQL schema and resolvers for balance queries
- Query service for handling balance requests

### Supporting Components

- Configuration management
- Error handling system
- Common utilities for Ethereum and decimal operations
- Type definitions shared across layers

## Implementation Notes

- Each layer should be loosely coupled to allow independent development and testing
- Use dependency injection for services to facilitate testing
- Follow Rust's module system to organize code logically
- Implement proper error handling throughout the system
- Create comprehensive tests for each component