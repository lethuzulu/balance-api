# File and Folder Structure for Phase 2: Multi-Chain Support

## Root Directory Structure

```
balances_api/
├── src/
│   ├── data_collection/
│   ├── storage/
│   ├── processing/
│   ├── query/
│   ├── common/
│   ├── chains/              # New directory for chain-specific code
│   └── main.rs
├── config/
├── tests/
├── Cargo.toml
├── docker-compose.yml
└── README.md

```

## Detailed Component Structure

### Chain-Specific Components

```
src/chains/
├── mod.rs                         # Module exports
├── chain_registry.rs              # Registry of supported chains
├── types.rs                       # Chain-specific type definitions
├── ethereum/
│   ├── mod.rs                     # Module exports
│   ├── config.rs                  # Ethereum chain configuration
│   └── utils.rs                   # Ethereum-specific utilities
├── polygon/
│   ├── mod.rs                     # Module exports
│   ├── config.rs                  # Polygon chain configuration
│   └── utils.rs                   # Polygon-specific utilities
└── arbitrum/
    ├── mod.rs                     # Module exports
    ├── config.rs                  # Arbitrum chain configuration
    └── utils.rs                   # Arbitrum-specific utilities

```

### Data Collection Layer (Enhanced for Multi-Chain)

```
src/data_collection/
├── mod.rs                         # Module exports
├── hypersync/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # Enhanced multi-chain HyperSync client
│   ├── transaction_collector.rs   # Multi-chain transaction collector
│   └── config.rs                  # Multi-chain HyperSync configuration
├── models/
│   ├── mod.rs                     # Module exports
│   ├── transaction.rs             # Enhanced transaction model with chain_id
│   └── block.rs                   # Enhanced block model with chain_id
├── kafka/
│   ├── mod.rs                     # Module exports
│   └── producer.rs                # Kafka producer with chain partitioning
├── chain_sync/
│   ├── mod.rs                     # Module exports
│   ├── sync_manager.rs            # Manages sync across multiple chains
│   └── sync_state.rs              # Tracks sync state per chain
└── service.rs                     # Multi-chain data collection service

```

### Storage Layer (Enhanced for Multi-Chain)

```
src/storage/
├── mod.rs                         # Module exports
├── clickhouse/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # ClickHouse client
│   ├── schema.rs                  # Enhanced schema with chain support
│   └── migrations/                # Database migrations
│       ├── mod.rs
│       ├── v1_initial_setup.rs    # Initial schema from Phase 1
│       └── v2_multi_chain.rs      # Multi-chain schema additions
├── redis/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # Redis client
│   └── cache.rs                   # Enhanced caching with chain keys
├── models/
│   ├── mod.rs                     # Module exports
│   ├── transaction_record.rs      # Enhanced with chain_id
│   └── balance_record.rs          # Enhanced with chain_id
└── repositories/
    ├── mod.rs                     # Module exports
    ├── transaction_repository.rs  # Multi-chain transaction repository
    └── balance_repository.rs      # Multi-chain balance repository

```

### Processing Layer (Enhanced for Multi-Chain)

```
src/processing/
├── mod.rs                         # Module exports
├── kafka/
│   ├── mod.rs                     # Module exports
│   └── consumer.rs                # Kafka consumer with chain-aware processing
├── processors/
│   ├── mod.rs                     # Module exports
│   ├── transaction_processor.rs   # Multi-chain transaction processor
│   ├── balance_processor.rs       # Multi-chain balance processor
│   └── chain_specific/            # Chain-specific processing logic if needed
│       ├── mod.rs
│       ├── ethereum_processor.rs
│       ├── polygon_processor.rs
│       └── arbitrum_processor.rs
├── models/
│   ├── mod.rs                     # Module exports
│   └── balance_change.rs          # Enhanced with chain_id
└── service.rs                     # Multi-chain processing service

```

### Query Layer (Enhanced for Multi-Chain)

```
src/query/
├── mod.rs                         # Module exports
├── grpc/
│   ├── mod.rs                     # Module exports
│   ├── server.rs                  # gRPC server implementation
│   ├── balance_service.rs         # Enhanced balance service with chain support
│   └── proto/                     # Protocol buffer definitions
│       ├── balance.proto          # Enhanced with chain parameters
│       └── generated/             # Generated code from protobuf
├── graphql/
│   ├── mod.rs                     # Module exports
│   ├── schema.rs                  # Enhanced GraphQL schema with chain support
│   ├── resolvers/                 # GraphQL resolvers
│   │   ├── mod.rs
│   │   └── balance_resolver.rs    # Multi-chain balance resolver
│   └── server.rs                  # GraphQL server
├── rest/                          # Optional REST API for simpler queries
│   ├── mod.rs                     # Module exports
│   ├── routes.rs                  # REST route definitions
│   ├── handlers.rs                # Request handlers
│   └── server.rs                  # REST server
└── service.rs                     # Multi-chain query service

```

### Common Code (Enhanced for Multi-Chain)

```
src/common/
├── mod.rs                         # Module exports
├── types.rs                       # Common type definitions with chain support
├── utils/
│   ├── mod.rs                     # Module exports
│   ├── ethereum.rs                # Ethereum-specific utilities
│   ├── address.rs                 # Cross-chain address handling
│   └── decimal.rs                 # Decimal handling utilities
├── error.rs                       # Enhanced error handling
└── config.rs                      # Multi-chain configuration management

```

### Configuration Files (Enhanced for Multi-Chain)

```
config/
├── default.toml                   # Default configuration with multi-chain settings
├── chains/                        # Chain-specific configurations
│   ├── ethereum.toml              # Ethereum configuration
│   ├── polygon.toml               # Polygon configuration
│   └── arbitrum.toml              # Arbitrum configuration
├── development.toml               # Development environment config
├── production.toml                # Production environment config
└── test.toml                      # Test environment config

```

### Test Directory (Enhanced for Multi-Chain)

```
tests/
├── integration/                   # Integration tests
│   ├── data_collection_tests.rs   # Multi-chain data collection tests
│   ├── storage_tests.rs           # Multi-chain storage layer tests
│   ├── processing_tests.rs        # Multi-chain processing layer tests
│   └── query_tests.rs             # Multi-chain query layer tests
├── chains/                        # Chain-specific tests
│   ├── ethereum_tests.rs          # Ethereum-specific tests
│   ├── polygon_tests.rs           # Polygon-specific tests
│   └── arbitrum_tests.rs          # Arbitrum-specific tests
└── e2e/                           # End-to-end tests
    ├── single_chain_test.rs       # Single chain E2E test
    └── multi_chain_test.rs        # Multi-chain E2E test

```

## Key Components to Implement in Phase 2

### Chain-Specific Components

- Chain registry for supported chains
- Chain-specific configurations and utilities
- Chain-type adapters and converters

### Data Collection Layer

- Enhanced HyperSync client with multi-chain support
- Chain-specific transaction collection
- Chain synchronization manager
- Multi-chain Kafka producers with proper partitioning

### Storage Layer

- Schema updates for chain_id in all tables
- Chain-specific partitioning strategies
- Multi-chain caching with appropriate key structures
- Enhanced repositories that handle chain-specific queries

### Processing Layer

- Chain-aware Kafka consumers
- Chain-specific transaction and balance processors
- Multi-chain balance calculation logic
- Chain-specific edge case handling

### Query Layer

- Enhanced API services with chain parameters
- Updated GraphQL schema with chain filtering
- Multi-chain query optimization
- Optional REST API for simple cross-chain queries

### Supporting Components

- Multi-chain configuration management
- Enhanced error handling for chain-specific issues
- Cross-chain address utilities
- Chain-aware logging and monitoring

## Implementation Notes

- Ensure all models have a chain_id field
- Use database partitioning by chain_id for performance
- Implement chain-specific validation where needed
- Create abstraction layers for chain-specific behaviors
- Set up proper error handling for chain-specific issues
- Consider data consistency across chains in the design
- Use proper serialization formats for cross-chain data