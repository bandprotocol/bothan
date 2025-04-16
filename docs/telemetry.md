# Telemetry

To provide better visibility into the status and activity of the system, Bothan includes a built-in telemetry service powered by the OpenTelemetry framework. Metrics are exposed over HTTP, making them easy to integrate with Prometheus for monitoring and observability.

---
## Configuration
The telemetry service is not active by default, and must be enabled in Bothan's configuration:

```toml
[telemetry]
enabled = false
addr = "127.0.0.1:4318"
```
---

## Metrics Overview

### 1. Server Request Metrics

Tracks each incoming Bothan gRPC server request and measures the processing time for each function call.

#### Metrics
| Name                                 | Type      | Description                                      |
|--------------------------------------|-----------|--------------------------------------------------|
| `server_requests_total`                    | `u64` Counter   | Total number of requests sent to the server           |
| `server_request_duration_milliseconds` | `u64` Histogram | Time taken to process each request (ms)     |

#### Tags
- `service_name`: `get_info`, `update_registry`, `push_monitoring_records`, `get_prices`
- `status`: gRPC response code, e.g., `ok`, `unavailable`, `internal`, `not_found`, and others.



---

### 2. Store Metrics

Tracks each operation that reads and writes asset information to RocksDB.

#### Metrics
| Name                              | Type      | Description                                                           |
|-----------------------------------|-----------|-----------------------------------------------------------------------|
| `store_operations_total`                    | `u64` Counter   | Total number of operations executed by the store
| `store_operation_duration_microseconds` | `u64` Histogram | Time taken to execute each store operation (μs)                    |

#### Tags
- `operation`: `get_asset_info`, `insert_batch_asset_info`
- `operation_status`: `success`, `not_found`, `failed`
---

### 3. REST Worker Metrics
Tracks workers that poll REST APIs from sources such as Coingecko and CoinMarketCap to retrieve asset information.

#### Metrics
| Name                              | Type      | Description                                                           |
|-----------------------------------|-----------|-----------------------------------------------------------------------|
| `rest_polling_total`                    | `u64` Counter   | Total number of polling requests sent by the worker to the source
| `rest_polling_duration_milliseconds` | `u64` Histogram | Time taken by the worker to complete each polling request to fetch asset info (ms)                    |

#### Tags
- `status`: `success`, `failed`, `timeout`
---

### 4. WebSocket Worker Metrics

Used by workers that establish WebSocket connections to each source: Binance, Bybit, Coinbase, HTX, Kraken, and OKX.

#### Metrics
| Name                                     | Type      | Description                                                                |
|------------------------------------------|-----------|----------------------------------------------------------------------------|
| `websocket_activity_messages_total`            | `u64` Counter   | Total number of messages sent by the source to indicate whether the source is active or not  |
| `websocket_connection_duration_milliseconds` | `u64` Histogram | Time taken for worker to establish a websocket connection to the source (ms)                              |
| `websocket_connection_total`                  | `u64` Counter   | Total number of connections established by a worker to the source                              |

#### Tags
- `worker`: 
   - For sources that require multiple connections: `{source_name}_{connection_idx}` (e.g., `binance_0`, `binance_1`, ...)
   - For single-connection sources: `source_name` (e.g., `binance`)
- `message_type`: `asset_info` or `ping`
- `status`: `success` or `failed`

---
## Integration with Prometheus
With `enabled = true` set for telemetry in your config.toml, the telemetry service will be activated and will expose metrics using the Prometheus encoder over HTTP at `http://localhost:4318/metrics`.

After starting Bothan with `bothan start`, and once the worker has been built and is receiving incoming gRPC requests, open `http://localhost:4318/metrics` in your browser — you should see the Prometheus-formatted metrics.

---
## Grafana Dashboard
Grafana provides a Bothan Dashboard template to visualize metrics efficiently. You can download and import the dashboard from Grafana's official repository.

[Download the Bothan Template Dashboard](https://grafana.com/grafana/dashboards/23176-bothan/)
