# Solana MEV Classifier

A Solana transaction inspection and classification tool that provides insight into MEV occuring block by block.

### Features

- Transaction inspection and parsing
- Classification of transaction types
- Identification of MEV (atomic arbitrage + sandwiches)
- API server for integration with other tools
- Command-line interface for direct usage

### Next Steps

- Add support for MEV done via CPI. Namely, by checking the ALT and the balance deltas on the relevant pools and addresses. 
- Populate DB and aggregate historical data
- Liquidation classifier

### Usage

#### Command Line Interface

Inspect a transaction:
```
cargo run --bin cli -- inspect [OPTIONS] <TRANSACTION_SIGNATURE>
```

Start the API server:
```
cargo run --bin cli -- serve [OPTIONS]
```

## Project Structure

- `packages/`: Core functionality modules
  - `actions/`: Transaction action definitions
  - `api/`: REST API implementation
  - `cli/`: Command-line interface
  - `inspection/`: Transaction inspection logic
  - `classifier-core/`: Core classification functionality
  - `action-tree/`: Action tree representation

- `classifiers/`: Transaction classification modules
  - `solana-classifier/`: Core Solana transaction classifier
  - `anchor-classifiers/`: Classifiers for Anchor-based programs
  - `misc-classifiers/`: Additional specialized classifiers