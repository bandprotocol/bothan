# price-adapter

`price-adapter` is a Rust library that provides different services for fetching price information for various cryptocurrencies and assets. It includes sources for popular data providers such as CoinGecko and Binance, and offers both HTTP-based and WebSocket-based interfaces for retrieving prices.

## Features

* **Multiple data sources:** `price-adapter` supports multiple sources for obtaining cryptocurrency prices, including:
    * CoinGecko
    * Binance
    * BandStableCoin
* **HTTP and WebSocket interfaces:** You can fetch prices either through HTTP requests or WebSocket connections, depending on your application's requirements.
* **Caching and interval-based updating:** The library features caching and interval-based updating of prices to optimize performance and reduce API calls.
* **Extensibility:** It's easy to add new data sources or implement custom mapping rules to adapt the library to your specific needs.

## Getting Started

To use `price-adapter`, add the following to your Cargo.toml file:

```toml
[dependencies]
price-adapter = "0.1.0"
```

## Services

`price-adapter` offers two types of services:

1. **IntervalService:** This service fetches prices from a specified source at regular intervals and caches them. This is a convenient option if you need to access prices frequently and don't want to make API calls every time.

2. **WebsocketService:** This service establishes a WebSocket connection to a source and subscribes to specific symbols. It then streams price updates over the WebSocket connection, allowing you to receive real-time price changes.

Both services implement the `Service` trait, which defines the common interface for starting, stopping, and checking the status of the service.

## Examples

The `examples` directory contains several example scripts demonstrating how to use `price-adapter`. These examples cover scenarios such as using the HTTP and WebSocket interfaces, creating a custom mapper, and employing the interval and Websocket services.
