# File and Folder Structure for Phase 3: Edge Case Handling

## Root Directory Structure

```
balances_api/
├── src/
│   ├── data_collection/
│   ├── storage/
│   ├── processing/
│   ├── query/
│   ├── common/
│   ├── chains/
│   ├── edge_cases/              # New directory for edge case handling
│   └── main.rs
├── config/
├── tests/
├── Cargo.toml
├── docker-compose.yml
└── README.md

```

## Detailed Component Structure

### Edge Case Components

```
src/edge_cases/
├── mod.rs                         # Module exports
├── detector.rs                    # Edge case detection orchestrator
├── types.rs                       # Edge case type definitions
├── self_destruct/
│   ├── mod.rs                     # Module exports
│   ├── detector.rs                # Self-destruct event detector
│   └── processor.rs               # Self-destruct event processor
├── internal_transactions/
│   ├── mod.rs                     # Module exports
│   ├── collector.rs               # Internal transaction collector
│   └── processor.rs               # Internal transaction processor
├── contract_lifecycle/
│   ├── mod.rs                     # Module exports
│   ├── tracker.rs                 # Contract creation/destruction tracker
│   └── analyzer.rs                # Contract lifecycle analyzer
└── validation/
    ├── mod.rs                     # Module exports
    ├── reconciliation.rs          # Balance reconciliation system
    └── edge_case_validator.rs     # Validator for edge case handling

```

### Data Collection Layer (Enhanced for Edge Cases)

```
src/data_collection/
├── mod.rs                         # Module exports
├── hypersync/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # HyperSync client
│   ├── transaction_collector.rs   # Transaction collector
│   ├── trace_collector.rs         # New: Trace collection for internal txs
│   └── config.rs                  # HyperSync configuration
├── models/
│   ├── mod.rs                     # Module exports
│   ├── transaction.rs             # Transaction model
│   ├── block.rs                   # Block model
│   ├── trace.rs                   # New: Trace model for internal txs
│   ├── internal_transaction.rs    # New: Internal transaction model
│   └── contract_event.rs          # New: Contract lifecycle events
├── kafka/
│   ├── mod.rs                     # Module exports
│   └── producer.rs                # Enhanced producer for edge case events
├── chain_sync/
│   ├── mod.rs                     # Module exports
│   ├── sync_manager.rs            # Sync manager
│   └── sync_state.rs              # Sync state tracker
└── service.rs                     # Enhanced data collection service

```

### Storage Layer (Enhanced for Edge Cases)

```
src/storage/
├── mod.rs                         # Module exports
├── clickhouse/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # ClickHouse client
│   ├── schema.rs                  # Enhanced schema with edge case tables
│   └── migrations/                # Database migrations
│       ├── mod.rs
│       ├── v1_initial_setup.rs    # Phase 1 schema
│       ├── v2_multi_chain.rs      # Phase 2 schema
│       └── v3_edge_cases.rs       # New edge case schema additions
├── redis/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # Redis client
│   └── cache.rs                   # Enhanced caching with edge case awareness
├── models/
│   ├── mod.rs                     # Module exports
│   ├── transaction_record.rs      # Transaction record model
│   ├── balance_record.rs          # Balance record model
│   ├── internal_tx_record.rs      # New: Internal transaction record
│   └── contract_lifecycle_record.rs # New: Contract lifecycle record
└── repositories/
    ├── mod.rs                     # Module exports
    ├── transaction_repository.rs  # Transaction repository
    ├── balance_repository.rs      # Balance repository
    ├── internal_tx_repository.rs  # New: Internal transaction repository
    └── contract_repository.rs     # New: Contract lifecycle repository

```

### Processing Layer (Enhanced for Edge Cases)

```
src/processing/
├── mod.rs                         # Module exports
├── kafka/
│   ├── mod.rs                     # Module exports
│   └── consumer.rs                # Enhanced consumer for edge case events
├── processors/
│   ├── mod.rs                     # Module exports
│   ├── transaction_processor.rs   # Transaction processor
│   ├── balance_processor.rs       # Balance processor
│   ├── internal_tx_processor.rs   # New: Internal transaction processor
│   ├── self_destruct_processor.rs # New: Self-destruct event processor
│   └── contract_processor.rs      # New: Contract lifecycle processor
├── state/
│   ├── mod.rs                     # Module exports
│   ├── balance_state.rs           # Balance state manager
│   └── contract_state.rs          # New: Contract state manager
├── models/
│   ├── mod.rs                     # Module exports
│   └── balance_change.rs          # Enhanced with edge case sources
└── service.rs                     # Enhanced processing service

```

### Query Layer (Enhanced for Edge Cases)

```
src/query/
├── mod.rs                         # Module exports
├── grpc/
│   ├── mod.rs                     # Module exports
│   ├── server.rs                  # gRPC server implementation
│   ├── balance_service.rs         # Enhanced balance service
│   ├── contract_service.rs        # New: Contract lifecycle service
│   └── proto/                     # Protocol buffer definitions
│       ├── balance.proto          # Enhanced balance service definition
│       ├── contract.proto         # New: Contract service definition
│       └── generated/             # Generated code from protobuf
├── graphql/
│   ├── mod.rs                     # Module exports
│   ├── schema.rs                  # Enhanced schema with edge case types
│   ├── resolvers/                 # GraphQL resolvers
│   │   ├── mod.rs
│   │   ├── balance_resolver.rs    # Enhanced balance resolver
│   │   └── contract_resolver.rs   # New: Contract lifecycle resolver
│   └── server.rs                  # GraphQL server
├── rest/                          # REST API
│   ├── mod.rs                     # Module exports
│   ├── routes.rs                  # Enhanced with edge case routes
│   ├── handlers.rs                # Enhanced handlers
│   └── server.rs                  # REST server
└── service.rs                     # Enhanced query service

```

### Common Code (Enhanced for Edge Cases)

```
src/common/
├── mod.rs                         # Module exports
├── types.rs                       # Common type definitions
├── utils/
│   ├── mod.rs                     # Module exports
│   ├── ethereum.rs                # Ethereum-specific utilities
│   ├── address.rs                 # Cross-chain address handling
│   ├── trace.rs                   # New: Trace analysis utilities
│   └── decimal.rs                 # Decimal handling utilities
├── error.rs                       # Enhanced error handling
└── config.rs                      # Enhanced configuration

```

### Configuration Files (Enhanced for Edge Cases)

```
config/
├── default.toml                   # Default configuration
├── chains/                        # Chain-specific configurations
│   ├── ethereum.toml              # Ethereum configuration
│   ├── polygon.toml               # Polygon configuration
│   └── arbitrum.toml              # Arbitrum configuration
├── edge_cases/                    # New: Edge case configurations
│   ├── detector.toml              # Edge case detection settings
│   ├── processing.toml            # Edge case processing settings
│   └── validation.toml            # Validation and reconciliation settings
├── development.toml               # Development environment config
├── production.toml                # Production environment config
└── test.toml                      # Test environment config

```

### Test Directory (Enhanced for Edge Cases)

```
tests/
├── integration/                   # Integration tests
│   ├── data_collection_tests.rs   # Data collection tests
│   ├── storage_tests.rs           # Storage layer tests
│   ├── processing_tests.rs        # Processing layer tests
│   ├── edge_case_tests.rs         # New: Edge case handling tests
│   └── query_tests.rs             # Query layer tests
├── edge_cases/                    # New: Edge case-specific tests
│   ├── self_destruct_tests.rs     # Self-destruct tests
│   ├── internal_tx_tests.rs       # Internal transaction tests
│   └── contract_lifecycle_tests.rs # Contract lifecycle tests
└── e2e/                           # End-to-end tests
    ├── balance_tracking_test.rs   # Balance tracking test
    └── edge_case_handling_test.rs # New: Edge case handling E2E test

```

## Key Components to Implement in Phase 3

### Edge Case Detection

- Self-destruct event detector
- Internal transaction collector
- Contract lifecycle tracker
- Edge case validation system

### Data Collection Enhancements

- Trace collection for internal transactions
- Extended transaction event models
- Contract creation/destruction event collection
- Enhanced Kafka producers for new event types

### Storage Enhancements

- Schema extensions for internal transactions
- Contract lifecycle tables
- Specialized indexes for edge case queries
- Enhanced caching strategies

### Processing Enhancements

- Internal transaction processor
- Self-destruct event handler
- Contract lifecycle state management
- Advanced balance calculation with edge cases

### Query Enhancements

- Edge case-aware balance API
- Contract lifecycle queries
- Enhanced balance reconciliation endpoints
- Visualization endpoints for transaction trees

### Supporting Components

- Trace analysis utilities
- Edge case monitoring system
- Reconciliation mechanisms
- Enhanced error handling for edge cases

## Implementation Notes

- Ensure comprehensive edge case detection with no false negatives
- Use proper transaction event ordering to maintain balance consistency
- Implement thorough validation and reconciliation systems
- Create detailed logging for edge case detection and processing
- Design the system for transparent tracking of balance changes from all sources
- Use optimized database queries for edge case-heavy addresses
- Develop comprehensive test suite with real-world edge cases