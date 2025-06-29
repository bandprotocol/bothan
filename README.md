# Bothan

[![MIT Licensed][mit-license-image]][mit-license-link]
[![Apache 2.0 Licensed][apache-license-image]][apache-license-link]
![Rust 1.85+][rustc-version]

A high-performance cryptocurrency price data aggregation system built in Rust.

## Overview

Bothan is a collection of libraries for querying and interacting with various cryptocurrency price data APIs. It
provides a unified interface for accessing real-time and historical cryptocurrency price data from multiple exchanges
and data providers.

## Project Structure

This project comprises primarily of 6 main components:

- [`bothan-core`](bothan-core/) - Core functionality and shared types
- [`bothan-lib`](bothan-lib/) - Common utilities and shared code
- [`bothan-api`](bothan-api/) - Main API server implementation
- [`bothan-api-proxy`](bothan-api-proxy/) - API proxy for load balancing and caching
- `bothan-{exchange}` - Exchange-specific implementations
- [`proto`](proto/) - Protocol buffer definitions

## Supported Data Sources

- [Binance](bothan-binance)
- [Bitfinex](bothan-bitfinex)
- [Bybit](bothan-bybit)
- [Coinbase](bothan-coinbase)
- [CoinGecko](bothan-coingecko)
- [CoinMarketCap](bothan-coinmarketcap)
- [CryptoCompare](bothan-cryptocompare)
- [HTX](bothan-htx)
- [Kraken](bothan-kraken)
- [OKX](bothan-okx)

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

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Running the API Stack

1. Configure the services:
    - Copy `bothan-api/server/config.toml.example` to `bothan-api/server/config.toml`
    - Update the configuration files with your API keys and settings

2. Start the services:
   ```bash
   docker-compose up
   ```

## Support

If you encounter any issues or have questions related to Bothan, we encourage you to open a GitHub issue. This ensures a
streamlined process for tracking and resolving problems.

Please use the following link to report your issue: https://github.com/bandprotocol/bothan/issues/new

## Contributing

We welcome contributions from the community! Please read open a Pull Request and follow the checklist in the
pull request template.

## Resources

- [Architecture Documentation](docs/architecture.md)
- [Build and Run Locally](docs/build_and_run_locally.md)
- [Telemetry Documentation](docs/telemetry.md)

## License

Copyright © 2024 Informal Systems Inc. and Bothan authors.

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

[mit-license-image]: https://img.shields.io/badge/license-MIT-blue

[mit-license-link]: LICENSE-MIT

[apache-license-image]: https://img.shields.io/badge/License-Apache_2.0-blue.svg

[apache-license-link]: LICENSE-APACHE

[rustc-version]: https://img.shields.io/badge/rustc-1.85+-blue.svg
