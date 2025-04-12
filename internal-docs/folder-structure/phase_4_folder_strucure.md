# File and Folder Structure for Phase 4: EOA Support

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
│   ├── edge_cases/
│   ├── eoa/                      # New directory for EOA-specific code
│   └── main.rs
├── config/
├── tests/
├── Cargo.toml
├── docker-compose.yml
└── README.md

```

## Detailed Component Structure

### EOA-Specific Components

```
src/eoa/
├── mod.rs                         # Module exports
├── types.rs                       # EOA-specific type definitions
├── mining/
│   ├── mod.rs                     # Module exports
│   ├── reward_collector.rs        # Mining reward collector
│   └── reward_processor.rs        # Mining reward processor
├── staking/
│   ├── mod.rs                     # Module exports
│   ├── event_collector.rs         # Staking event collector
│   └── reward_processor.rs        # Staking reward processor
├── gas/
│   ├── mod.rs                     # Module exports
│   ├── tracker.rs                 # Gas expenditure tracker
│   └── analytics.rs               # Gas usage analytics
├── analytics/
│   ├── mod.rs                     # Module exports
│   ├── transaction_patterns.rs    # Transaction pattern analyzer
│   └── activity_aggregator.rs     # Account activity aggregator
└── security/
    ├── mod.rs                     # Module exports
    ├── anomaly_detector.rs        # Anomalous activity detector
    └── risk_analyzer.rs           # Security risk analyzer

```

### Data Collection Layer (Enhanced for EOA)

```
src/data_collection/
├── mod.rs                         # Module exports
├── hypersync/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # HyperSync client
│   ├── transaction_collector.rs   # Transaction collector
│   ├── trace_collector.rs         # Trace collection
│   ├── block_reward_collector.rs  # New: Block reward collector
│   └── config.rs                  # HyperSync configuration
├── models/
│   ├── mod.rs                     # Module exports
│   ├── transaction.rs             # Transaction model
│   ├── block.rs                   # Block model
│   ├── trace.rs                   # Trace model
│   ├── mining_reward.rs           # New: Mining reward model
│   ├── staking_event.rs           # New: Staking event model
│   └── gas_usage.rs               # New: Gas usage model
├── kafka/
│   ├── mod.rs                     # Module exports
│   └── producer.rs                # Enhanced producer for EOA events
├── chain_sync/
│   ├── mod.rs                     # Module exports
│   ├── sync_manager.rs            # Sync manager
│   └── sync_state.rs              # Sync state tracker
└── service.rs                     # Enhanced data collection service

```

### Storage Layer (Enhanced for EOA)

```
src/storage/
├── mod.rs                         # Module exports
├── clickhouse/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # ClickHouse client
│   ├── schema.rs                  # Enhanced schema with EOA tables
│   └── migrations/                # Database migrations
│       ├── mod.rs
│       ├── v1_initial_setup.rs    # Phase 1 schema
│       ├── v2_multi_chain.rs      # Phase 2 schema
│       ├── v3_edge_cases.rs       # Phase 3 schema
│       └── v4_eoa_support.rs      # New EOA schema additions
├── redis/
│   ├── mod.rs                     # Module exports
│   ├── client.rs                  # Redis client
│   └── cache.rs                   # Enhanced caching for EOA data
├── models/
│   ├── mod.rs                     # Module exports
│   ├── transaction_record.rs      # Transaction record model
│   ├── balance_record.rs          # Balance record model
│   ├── mining_reward_record.rs    # New: Mining reward record model
│   ├── staking_event_record.rs    # New: Staking event record model
│   └── gas_usage_record.rs        # New: Gas usage record model
└── repositories/
    ├── mod.rs                     # Module exports
    ├── transaction_repository.rs  # Transaction repository
    ├── balance_repository.rs      # Balance repository
    ├── mining_reward_repository.rs # New: Mining reward repository
    ├── staking_repository.rs      # New: Staking event repository
    └── gas_usage_repository.rs    # New: Gas usage repository

```

### Processing Layer (Enhanced for EOA)

```
src/processing/
├── mod.rs                         # Module exports
├── kafka/
│   ├── mod.rs                     # Module exports
│   └── consumer.rs                # Enhanced consumer for EOA events
├── processors/
│   ├── mod.rs                     # Module exports
│   ├── transaction_processor.rs   # Transaction processor
│   ├── balance_processor.rs       # Balance processor
│   ├── mining_reward_processor.rs # New: Mining reward processor
│   ├── staking_processor.rs       # New: Staking event processor
│   └── gas_processor.rs           # New: Gas usage processor
├── analytics/
│   ├── mod.rs                     # Module exports
│   ├── transaction_analyzer.rs    # New: Transaction pattern analyzer
│   └── account_activity.rs        # New: Account activity aggregator
├── models/
│   ├── mod.rs                     # Module exports
│   ├── balance_change.rs          # Balance change model
│   └── comprehensive_balance.rs   # New: Comprehensive balance model
└── service.rs                     # Enhanced processing service

```

### Query Layer (Enhanced for EOA)

```
src/query/
├── mod.rs                         # Module exports
├── grpc/
│   ├── mod.rs                     # Module exports
│   ├── server.rs                  # gRPC server implementation
│   ├── balance_service.rs         # Enhanced balance service
│   ├── mining_service.rs          # New: Mining reward service
│   ├── staking_service.rs         # New: Staking event service
│   ├── analytics_service.rs       # New: Analytics service
│   └── proto/                     # Protocol buffer definitions
│       ├── balance.proto          # Enhanced balance service definition
│       ├── mining.proto           # New: Mining service definition
│       ├── staking.proto          # New: Staking service definition
│       ├── analytics.proto        # New: Analytics service definition
│       └── generated/             # Generated code from protobuf
├── graphql/
│   ├── mod.rs                     # Module exports
│   ├── schema.rs                  # Enhanced schema with EOA types
│   ├── resolvers/                 # GraphQL resolvers
│   │   ├── mod.rs
│   │   ├── balance_resolver.rs    # Enhanced balance resolver
│   │   ├── mining_resolver.rs     # New: Mining reward resolver
│   │   ├── staking_resolver.rs    # New: Staking event resolver
│   │   └── analytics_resolver.rs  # New: Analytics resolver
│   └── server.rs                  # GraphQL server
├── rest/                          # REST API
│   ├── mod.rs                     # Module exports
│   ├── routes.rs                  # Enhanced with EOA routes
│   ├── handlers.rs                # Enhanced handlers
│   └── server.rs                  # REST server
└── service.rs                     # Enhanced query service

```

### Common Code (Enhanced for EOA)

```
src/common/
├── mod.rs                         # Module exports
├── types.rs                       # Common type definitions
├── utils/
│   ├── mod.rs                     # Module exports
│   ├── ethereum.rs                # Ethereum-specific utilities
│   ├── address.rs                 # Cross-chain address handling
│   ├── mining.rs                  # New: Mining reward utilities
│   ├── staking.rs                 # New: Staking utilities
│   └── decimal.rs                 # Decimal handling utilities
├── error.rs                       # Enhanced error handling
└── config.rs                      # Enhanced configuration

```

### Configuration Files (Enhanced for EOA)

```
config/
├── default.toml                   # Default configuration
├── chains/                        # Chain-specific configurations
│   ├── ethereum.toml              # Ethereum configuration
│   ├── polygon.toml               # Polygon configuration
│   └── arbitrum.toml              # Arbitrum configuration
├── edge_cases/                    # Edge case configurations
│   ├── detector.toml              # Edge case detection settings
│   ├── processing.toml            # Edge case processing settings
│   └── validation.toml            # Validation settings
├── eoa/                           # New: EOA configurations
│   ├── mining.toml                # Mining reward settings
│   ├── staking.toml               # Staking event settings
│   ├── gas.toml                   # Gas tracking settings
│   └── analytics.toml             # Analytics settings
├── development.toml               # Development environment config
├── production.toml                # Production environment config
└── test.toml                      # Test environment config

```

### Test Directory (Enhanced for EOA)

```
tests/
├── integration/                   # Integration tests
│   ├── data_collection_tests.rs   # Data collection tests
│   ├── storage_tests.rs           # Storage layer tests
│   ├── processing_tests.rs        # Processing layer tests
│   ├── query_tests.rs             # Query layer tests
│   └── eoa_tests.rs               # New: EOA support tests
├── eoa/                           # New: EOA-specific tests
│   ├── mining_tests.rs            # Mining reward tests
│   ├── staking_tests.rs           # Staking event tests
│   ├── gas_tests.rs               # Gas tracking tests
│   └── analytics_tests.rs         # Analytics tests
├── edge_cases/                    # Edge case tests
│   ├── self_destruct_tests.rs     # Self-destruct tests
│   ├── internal_tx_tests.rs       # Internal transaction tests
│   └── contract_lifecycle_tests.rs # Contract lifecycle tests
└── e2e/                           # End-to-end tests
    ├── balance_tracking_test.rs   # Balance tracking test
    ├── edge_case_handling_test.rs # Edge case handling test
    └── eoa_complete_test.rs       # New: Complete EOA functionality test

```

## Key Components to Implement in Phase 4

### Mining Reward Tracking

- Mining reward collector for blocks mined by EOAs
- Uncle/nephew reward calculations
- Mining reward analytics and historical views
- Mining profitability calculators

### Staking Support

- Staking event collectors (deposits, rewards, slashing, withdrawals)
- Validator performance tracking
- Staking reward analytics
- Staking APR/APY calculators

### Gas Analytics

- Gas expenditure tracking per EOA
- Gas optimization recommendations
- Historical gas cost analysis
- Fee market analysis for optimal transaction timing

### Transaction Pattern Analysis

- Transaction frequency and volume analytics
- Recurring payment detection
- Counterparty analysis
- Address interaction networks

### Activity Aggregation

- Daily/weekly/monthly account summaries
- Activity categorization (DEX trades, transfers, etc.)
- Cross-chain activity consolidation
- Complete financial overview generation

### Security Monitoring

- Anomalous transaction detection
- Security risk scoring
- Suspicious activity alerts
- Large movement notifications

## Implementation Notes

- Implement proper reward calculation for mining and staking
- Design efficient aggregation mechanisms for high-frequency traders
- Create detailed historical views for all reward types
- Build comprehensive analytics with visualization endpoints
- Ensure cross-chain consistency in reward calculations
- Develop granular monitoring for high-value EOAs
- Create thorough and efficient testing for reward accuracy
- Implement proper handling of different reward mechanisms across chains