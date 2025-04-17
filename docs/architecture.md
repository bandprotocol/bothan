# Architecture

This document describes the architecture of Bothan, a high-performance cryptocurrency price data aggregation system built in Rust.

## Terms

Some important terms and acronyms that are commonly used in this project include:

* **Bothan**: The name of this cryptocurrency price data aggregation system.
* **Provider**: A service that provides cryptocurrency market data (prices, volumes, etc.). This includes both exchanges (like Binance, Kraken) and data aggregation services (like CoinMarketCap, CoinGecko).
* **Price Feed**: A continuous stream of price data for a specific cryptocurrency pair.

## Overview

At its highest level, Bothan implements a unified system for collecting, processing, and serving cryptocurrency price data from multiple sources. The system is designed to be highly performant, reliable, and extensible, using Rust's powerful type system and async capabilities through Tokio.

Bothan follows a modular architecture, with separate components for different providers and a common core that handles shared functionality. The system can be broken down into several key components:

1. **Core Components**: Shared types, interfaces, and functionality used across the system.
2. **Provider Connectors**: Specialized modules for interacting with individual providers.
3. **API Server**: The interface for external systems to request price data.
4. **API Proxy**: A REST interface layer that sits on top of the gRPC API implementation.

![Bothan Architecture Diagram](architecture.png)

## Component Descriptions

This section provides an overview of the various directories and modules in Bothan.

### `bothan-core`

The foundation of the system, containing core types, interfaces, and functionality shared across all components, including:

- Data models and schemas
- Storage interfaces
- Monitoring components
- IPFS integration
- Manager interfaces

### `bothan-lib`

Common utilities and shared code that doesn't fit into the core module but is used by multiple components:

- Worker abstractions for handling asynchronous tasks
- Type definitions and conversions
- Registry functionality for service discovery
- Storage implementations

### `bothan-api`

The main API server implementation, consisting of:

- Server: The main implementation of the API endpoints
- Server CLI: Command-line interface for running the API server
- Client: SDKs for interacting with the API from other applications

### `bothan-api-proxy`

A proxy layer that provides a REST interface over the gRPC implementation:

- Translates between REST and gRPC
- Simplifies client integration
- Provides HTTP access to the API
- Enables integration with web applications

### Provider-Specific Modules

Each provider has its own dedicated module:

- `bothan-binance`: Integration with Binance
- `bothan-bybit`: Integration with Bybit
- `bothan-coinbase`: Integration with Coinbase
- `bothan-coingecko`: Integration with CoinGecko
- `bothan-coinmarketcap`: Integration with CoinMarketCap
- `bothan-htx`: Integration with HTX
- `bothan-kraken`: Integration with Kraken
- `bothan-okx`: Integration with OKX
- `bothan-bitfinex`: Integration with Bitfinex

These modules implement provider-specific logic while conforming to common interfaces defined in the core components.

### `proto`

Protocol buffer definitions for the API, enabling:

- Consistent data serialization
- Language-agnostic interface definitions
- Efficient binary encoding

## System Aspects

### Async Runtime

Bothan uses Tokio as its async runtime, allowing it to efficiently handle thousands of concurrent connections with minimal resource usage. The async programming model is used throughout the codebase to ensure optimal performance.

### Error Handling

Error handling in Bothan follows Rust's idiomatic approach using the `Result` type. The system uses `anyhow` for flexible error handling and `thiserror` for defining custom error types where appropriate.

### Monitoring and Telemetry

Bothan includes comprehensive monitoring capabilities to track system health, performance metrics, and error rates. The monitoring system is designed to integrate with industry-standard tools.

### Testing

The codebase includes unit tests for core functionality, as well as integration tests to verify interactions with external systems. Mock implementations are available for testing provider connectors without making actual API calls.

## Extensions and Customization

Bothan is designed to be extensible, allowing for:

- Adding new providers by implementing the common interfaces
- Customizing data processing pipelines
- Extending the API with new endpoints
- Integrating with external systems

For details on how to contribute or extend the system, see the [Contributing Guide](contributing.md).
