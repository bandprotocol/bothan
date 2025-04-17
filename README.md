# Bothan


[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]
![Rust 1.76+][rustc-version]

A high-performance cryptocurrency price data aggregation system built in Rust.

## Overview

Bothan is a collection of libraries for querying and interacting with various cryptocurrency price data APIs. It provides a unified interface for accessing real-time and historical cryptocurrency price data from multiple exchanges and data providers.

## Project Structure

This project comprises primarily of 6 main components:

- [`bothan-core`](bothan-core/) - Core functionality and shared types
- [`bothan-lib`](bothan-lib/) - Common utilities and shared code
- [`bothan-api`](bothan-api/) - Main API server implementation
- [`bothan-api-proxy`](bothan-api-proxy/) - API proxy for load balancing and caching
- [`bothan-{exchange}`](bothan-binance/) - Exchange-specific implementations
- [`proto`](proto/) - Protocol buffer definitions

## Supported Data Sources

- [Binance](bothan-binance)
- [Bybit](bothan-bybit)
- [CoinGecko](bothan-coingecko)
- [CoinMarketCap](bothan-coinmarketcap)
- [CryptoCompare](bothan-cryptocompare)
- [HTX](bothan-htx)
- [Kraken](bothan-kraken)
- [OKX](bothan-okx)
- [Bitfinex](bothan-bitfinex)
- [Coinbase](bothan-coinbase)

## Features

- **Unified API**: Consistent interface across all supported exchanges
- **High Performance**: Built with Rust and Tokio for maximum efficiency
- **Real-time Data**: Support for streaming and websocket connections
- **Caching**: Built-in caching to minimize API calls
- **Load Balancing**: Distributed architecture for high availability
- **Rate Limiting**: Intelligent rate limit management
- **Error Handling**: Robust error handling and retry mechanisms

## Quick Start

### Prerequisites

Choose one of the following development approaches:

#### Using Docker (Recommended for quick setup)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

#### Using Rust (For development and customization)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Running the API Stack

1. Configure the services:
   - Copy `bothan-api/server/config.toml.example` to `bothan-api/server/config.toml`
   - Copy `bothan-api-proxy/config.toml.example` to `bothan-api-proxy/config.toml`
   - Update the configuration files with your API keys and settings

2. Start the services:
   ```bash
   docker-compose up
   ```

## Support

If you encounter any issues or have questions related to Bothan, we encourage you to open a GitHub issue. This ensures a streamlined process for tracking and resolving problems.

Please use the following link to report your issue: https://github.com/informalsystems/bothan/issues/new

## Contributing

We welcome contributions from the community! Please read our [Contributing Guidelines](docs/contributing.md) for detailed information on how to contribute to the project.

## Resources

- [Architecture Documentation](docs/architecture.md)
- [API Documentation](docs/api.md)
- [Configuration Guide](docs/configuration.md)
- [Contributing Guide](docs/contributing.md)

## License

Copyright © 2024 Informal Systems Inc. and Bothan authors.

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

[build-image]: https://placeholder
[build-link]: https://placeholder
[license-image]: https://placeholder
[license-link]: https://placeholder
[rustc-image]: https://img.shields.io/badge/rustc-stable-blue.svg
[rustc-version]: https://img.shields.io/badge/rustc-1.76+-blue.svg
