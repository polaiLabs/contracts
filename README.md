# Polai: Prediction Market Smart Contracts

Decentralized, non-custodial binary prediction market protocol built on **Stellar Soroban**.

## Features
- **Parimutuel Shared Pool**: Fully collateralized, zero-insolvency risk.
- **Fair Settlement**: Proportional payouts calculated dynamically on-chain.
- **Dispute-Aware Resolution**: Designated resolver and admin override safeguards.
- **Audit-Ready**: Storage TTL management and indexed event streams for indexers.

## Build & Test
```bash
# Run unit tests
cargo test

# Build WASM bytecode
stellar contract build
```

## Structure
- `contracts/prediction-market/src/lib.rs` - Core contract logic & storage structures
- `contracts/prediction-market/src/test.rs` - Lifecycle & edge-case test suite
