# Polai: Prediction Market Smart Contracts

Decentralized, non-custodial binary prediction market protocol built on **Stellar Soroban**.

## Features
- **Parimutuel Shared Pool**: Fully collateralized, zero-insolvency risk.
- **Fair Settlement**: Proportional payouts calculated dynamically on-chain.
- **Dispute-Aware Resolution**: Designated resolver and admin override safeguards.
- **Audit-Ready**: Storage TTL management and indexed event streams for indexers.

## Deployed Contract (Stellar Testnet)
- **Contract ID**: `CALWZ24CPYSPTZ3ID2LTDLBQWDJBUUL4UG4E77M6ZU4GAL7AYD67NZZP`
- **Admin & Resolver**: `GCBC42NQP56ZHPWYFR3GRET2B4ATHMK3UJPVP7ETVHJ73XFJVNEB24PM`
- **Explorer**: [Stellar Expert Explorer](https://stellar.expert/explorer/testnet/contract/CALWZ24CPYSPTZ3ID2LTDLBQWDJBUUL4UG4E77M6ZU4GAL7AYD67NZZP)
- **Stellar Lab**: [Stellar Lab Contract](https://lab.stellar.org/r/testnet/contract/CALWZ24CPYSPTZ3ID2LTDLBQWDJBUUL4UG4E77M6ZU4GAL7AYD67NZZP)

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
