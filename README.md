# Bothan

## Overview

`Bothan` is a collection of libraries for querying and interacting with various cryptocurrency price data APIs.

`Bothan` supports the following sources:

- [Binance](bothan-binance)
- [Bybit](bothan-bybit)
- [CoinGecko](bothan-coingecko)
- [CoinMarketCap](bothan-coinmarketcap)
- [CryptoCompare](bothan-cryptocompare)
- [HTX](bothan-htx)
- [Kraken](bothan-kraken)

## Bothan API Stack

### Pre-requisites

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Usage

To run the Bothan API stack, set the config.toml file in both the [bothan-api](bothan-api)
and [bothan-api-proxy](bothan-api-proxy) directories then run the following command:

```bash
docker-compose up
```

## Contributing

We welcome contributions from the community! Before submitting a pull request, please review
our [contributing guidelines](CONTRIBUTING.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
