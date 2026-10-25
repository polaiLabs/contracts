# Contributing to Polai Smart Contracts

Thank you for your interest in contributing to **Polai**! This repository hosts the Soroban smart contracts for the decentralized prediction market on Stellar.

## Getting Started

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (v1.75+)
- `wasm32-unknown-unknown` target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli):
  ```bash
  cargo install --locked stellar-cli
  ```

### Local Development & Testing
```bash
# Clone the repository
git clone https://github.com/polaiLabs/contracts.git
cd contracts

# Run unit and integration tests
cargo test

# Build WASM binary
stellar contract build
```

## Drips Wave Contributions
We welcome contributions during active **Drips Wave** sprints!

- Browse issues labeled with `Stellar Wave` or `good first issue`.
- Please apply for the issue in Drips Wave before submitting a PR.
- Ensure all tests pass (`cargo test`) and adhere to standard Rust formatting (`cargo fmt --check`).
- Document any new contract functions, storage keys, or event definitions.
