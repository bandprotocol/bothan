# Start from the latest Rust image
FROM rust:1.78-bookworm AS builder

# Create a new empty Rust project
RUN USER=root cargo new --bin bothan-api

# Copy entire source code
COPY ../.. .

# Build the project
RUN cargo build -p bothan-api --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*.
RUN ldconfig

# Copy the binary from builder
COPY --from=builder /target/release/bothan-api /usr/local/bin

# Copy registry
COPY --from=builder /bothan-api/server/registry /app/registry

WORKDIR /app
CMD ["bothan-api"]
