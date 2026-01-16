# cowSolver

[![CI](https://github.com/0xDevNinja/cowSolver/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/0xDevNinja/cowSolver/actions/workflows/ci.yml)

## Overview

cowSolver is a high-performance cross-chain solver implementation for the CoW Protocol. It enables seamless token swaps across multiple blockchain networks with optimal routing and settlement. The project is written in Rust and is composed of modular components that can be independently extended and tested.

## Architecture

The solver consists of several layers:

- **Solver Engine** – Handles batch auction processing and orchestration.
- **Order Matching** – Performs CoW discovery and matches orders via direct pairs and rings.
- **AMM Routing** – Computes optimal multi-hop routing through AMMs like Uniswap, Balancer and Curve.
- **Pricing Engine** – Calculates clearing prices using uniform or volume-weighted strategies.
- **Domain Models** – Defines structs for orders, tokens, chains and settlements.
- **Math Utilities** – Provides precise decimal arithmetic and slippage calculations.
- **Adapters Layer** – Implements chain RPC clients and DEX integrations for on-chain interactions.
- **Bridge Integration** – Executes cross-chain settlements using bridges.
- **Strategy Layer** – Houses solving strategies with pluggable heuristics.
- **CLI Binary** – Offers command-line interface for interacting with the solver.
- **Daemon Service** – Provides a long-running service with HTTP/gRPC APIs.
- **Integration Tests** – Validates end-to-end flows across modules.
- **Performance Optimizations** – Contains utilities for improving throughput and reducing gas.

See the [docs/architecture.md](docs/architecture.md) guide for a detailed diagram and explanation of how these components interact.

## Getting Started

### Prerequisites

- Rust 1.75 or later installed (`rustup toolchain install stable`)
- Cargo for building and testing
- (Optional) Docker for running local testnets

### Building and Running

Clone the repository and run:

```bash
git clone https://github.com/0xDevNinja/cowSolver.git
cd cowSolver
cargo build --release
cargo test
```

To start the daemon service:

```bash
cargo run --bin daemon -- --help
```

### CLI Usage

The CLI binary is located in `src/bin/cli.rs`. Build and run it:

```bash
cargo run --bin cli -- run
cargo run --bin cli -- submit --order-file examples/order.json
```

Use `--help` with any subcommand to see available options.

### Enabling Chainstore Fallback

Some features such as cross-chain settlement rely on external services. To enable the optional chainstore fallback for fetching missing blocks via Bitswap, set the following environment variable:

```bash
export FOREST_ENABLE_CHAINSTORE_FALLBACK=1
```

See the [Chainstore Fallback guide](docs/chainstore_fallback.md) for more details.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, coding guidelines, and instructions for submitting issues and pull requests.

## License

This project is licensed under the MIT License.
