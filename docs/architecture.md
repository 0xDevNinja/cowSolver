# Architecture

The **cowSolver** project is structured as a modular Rust crate with clear separation of concerns. The high-level components include:

- **Domain models** (`models`): Defines fundamental data structures like `Order`, `Token`, `ChainId`, and `Settlement`. These types model orders, tokens, blockchain identifiers, and settlement receipts.
- **Solver engine** (`solver`): Orchestrates batch auctions. It collects orders, groups them into auctions, and delegates matching, routing and pricing to other modules. The engine exposes traits (`BatchAuction`, `Solver`) to allow custom implementations.
- **Order matching** (`matching`): Implements algorithms to find complementary orders. Current implementations include `PairMatcher` and `RingMatcher`, which discover direct and cyclic match opportunities.
- **AMM routing** (`routing`): Computes optimal multi-hop routes across automated market makers such as Uniswap, Balancer and Curve. Each protocol has its own router that implements a common `AmmRouter` trait.
- **Pricing engine** (`pricing`): Calculates clearing prices for auctions. Strategies like uniform pricing and volume-weighted pricing can be added by implementing the `PricingEngine` trait.
- **Math utilities and performance** (`utils`, `performance`): Provide decimal arithmetic helpers, slippage calculations, normalization functions and sorting/optimization utilities.
- **Adapters** (`adapters`): Abstract RPC clients and decentralized exchange integrations. Traits like `ChainRpcClient` and `DexAdapter` allow plugging in real node clients and DEX interfaces.
- **Bridge integration** (`bridge`): Provides a `BridgeAdapter` trait for executing settlements across chains. A dummy adapter is included; real implementations can connect to protocols such as Hop or Connext.
- **Strategy layer** (`strategy`): Defines pluggable solver strategies (e.g., baseline, advanced). Each strategy implements the `Strategy` trait and dictates how orders are grouped and priced.
- **Daemon service** (`daemon`): A long-running service built with Tokio that periodically runs the solver and exposes an HTTP or gRPC API for managing orders and retrieving settlements.
- **Command-line interface** (`src/bin/cli.rs`): Offers a user-friendly CLI built with Clap to run the solver, submit orders, or execute integration tests.

## Data Flow

1. **Order ingestion:** Orders are created and validated using the `models` module and stored in an in-memory or persistent store.
2. **Batch formation:** The solver engine groups orders into batch auctions based on timing or external triggers.
3. **Matching:** For each auction, the `matching` module discovers matching sets of orders using pair and ring matching strategies.
4. **Routing:** The routing layer computes optimal swap paths across supported AMMs for each potential match.
5. **Pricing:** The pricing engine calculates a uniform clearing price (or other strategies) that satisfies all matches.
6. **Settlement:** The result is packaged into a `Settlement` structure and executed on chain. If the target chain differs from the order chain, the bridge adapter handles cross-chain execution.
7. **Performance & Metrics:** Utilities and performance modules collect metrics, optimize gas usage, and ensure the solver scales efficiently.

## Extensibility

The architecture is intentionally modular. To add new functionality:

- Implement additional `Matcher` strategies in `matching` for novel order matching patterns.
- Add new routers in `routing` to support other AMMs or DEX aggregators.
- Provide alternative pricing strategies by implementing `PricingEngine`.
- Integrate real blockchain nodes and DEXs by implementing `ChainRpcClient` and `DexAdapter`.
- Introduce real bridge protocols by implementing `BridgeAdapter`.
- Build more sophisticated solver strategies in the `strategy` layer.

This modular design facilitates industry-level adoption, allowing the solver to evolve alongside the rapidly changing decentralized finance landscape.
