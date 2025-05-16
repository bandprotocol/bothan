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

To run the Bothan API stack, set the config.toml file in both the [bothan-api](bothan-api/server)
and [bothan-api-proxy](bothan-api-proxy) directories. An example config
for [bothan-api](bothan-api/server/config.toml.example) and [bothan-api-proxy](bothan-api-proxy/config.toml.example) is
given. Once the config file is set, run the following command from the root directory.

```bash
docker-compose up
```

## Proto

### Pre-requisites

To generate the protobuf files, you need to have the following installed:

- [Buf](https://buf.build/docs/installation)

### Generate

To generate the protobuf files, run the following command:

```bash
buf generate
buf build -o bothan-api/server/src/proto/descriptor.pb
```

## Contributing

We welcome contributions from the community! Before submitting a pull request, please review
our [contributing guidelines](CONTRIBUTING.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
