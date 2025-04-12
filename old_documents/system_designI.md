# ETH Balance Tracking System Design

## System Overview

### Purpose
A scalable system to track real-time and historical ETH balances across multiple EVM chains, supporting both contract addresses and EOAs, with comprehensive edge case handling.

### Development Phases

#### Phase 1: Ethereum Mainnet Basic Implementation
- **Purpose**: Track running ETH balances for contract addresses on ethereum mainnet
- **Scope**: Basic contract balance tracking without edge cases

#### Phase 2: Multi-Chain Support
- **Purpose**: Track the running balance of the address on more than one EVM chain, include tracking balance on L2 chains for the same contract address
- **Scope**: Basic contract balance tracking on multiple chains without edge cases

#### Phase 3: Edge Case Handling
- **Purpose**: Handle all edge cases related to contract addresses
- **Scope**: Take into account all edge cases related to contract addresses

#### Phase 4: EOA Support
- **Purpose**: Add support for EOA addresses
- **Scope**: Full support for both contract and EOA addresses

## Core Components (Layers)

### 1. Data Collection Layer

#### Chain Integration Module
- **Phase 1**: Ethereum Mainnet only
- **Phase 2**: Multiple EVM chains + L2s
- **Components**:
  - Chain Connectors
  - Data Synchronization Controllers
  - Chain-specific Adapters

#### Transaction Collector
- **Capabilities**:
  - Incoming transactions
  - Outgoing transactions
  - Internal transactions (Phase 3)
  - L2 bridge transactions (Phase 2)

### 2. Storage Layer

#### Multi-Chain Data Store
- **Schema Design**:
  ```
  Chains
  ├── chain_id
  ├── chain_type (L1/L2)
  └── chain_config
  
  Addresses
  ├── address
  ├── address_type (Contract/EOA)
  ├── deployment_info
  └── supported_chains
  
  Balances
  ├── address
  ├── chain_id
  ├── block_number
  ├── timestamp
  ├── balance
  └── last_transaction_hash
  
  Transactions
  ├── chain_id
  ├── transaction_hash
  ├── block_number
  ├── from_address
  ├── to_address
  ├── value
  ├── transaction_type
  └── status
  ```

#### Index Management
- **Primary indices**:
  - (address, chain_id, block_number)
  - (chain_id, block_number)
  - (address, chain_id, timestamp)

### 3. Processing Layer

#### Balance Calculator
- **Components**:
  - Transaction Processor
  - Balance Updater
  - State Manager
  - Edge Case Handler (Phase 3)

#### Chain Orchestrator
- **Manages**:
  - Cross-chain synchronization
  - L2 bridge monitoring
  - Reorg handling

#### Validation Engine
- **Features**:
  - Balance verification
  - Cross-chain consistency
  - Edge case detection

### 4. Query Layer

#### Balance Query Service
- **Capabilities**:
  - Single chain balance
  - Multi-chain aggregation
  - Historical balance lookup
  - Running balance calculation

#### Analytics Engine
- **Features**:
  - Balance history
  - Transaction patterns
  - Cross-chain flows
  - Address activity metrics

## Phase Implementation Details

### Phase 1: Ethereum Mainnet Basic Implementation
- **Focus**: Single chain, contract addresses only
- **Key Components**:
  - Basic Hypersync integration
  - Simple balance tracking
  - Core storage schema
  - Basic query capabilities

### Phase 2: Multi-Chain Expansion
- **Additions**:
  - Multi-chain support
  - L2 integration
  - Bridge transaction tracking
  - Cross-chain balance aggregation
  - Chain-specific adapters

### Phase 3: Edge Case Handling
- **Enhancements**:
  - Self-destruct detection
  - Internal transaction tracking
  - Mining/Uncle rewards
  - Contract creation/destruction
  - Failed transaction handling

### Phase 4: EOA Support
- **Extensions**:
  - EOA detection
  - Mining rewards tracking
  - Account type-specific processing
  - Enhanced analytics

## Hypersync Integration

### Transaction Tracking
Using the following methods:
- transactions_from_address()
- blocks_and_transactions()

### Data Collection
Key fields from Transaction struct:
- value: Option<Quantity>
- from: Option<Address>
- to: Option<Address>
- block_number: Option<BlockNumber>
- status: Option<TransactionStatus>

### L2 Specific
L2 specific fields:
- l1_fee: Option<Quantity>
- l1_gas_price: Option<Quantity>
- l1_gas_used: Option<Quantity>