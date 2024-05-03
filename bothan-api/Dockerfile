# Start from the latest Rust image
FROM rust:bookworm as builder

# Create a new empty Rust project
RUN USER=root cargo new --bin bothan-api

# Copy the entire workspace
COPY . .

# Build the project
WORKDIR /bothan-api
RUN cargo build --release

# Copy the source code
COPY ./bothan-api/src ./src

# Rebuild the project
RUN cargo build -p bothan-api --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl && apt-get install -y ca-certificates
RUN ldconfig

# Copy the binary from builder
COPY --from=builder /target/release/bothan-api /usr/local/bin

# Copy registry
COPY --from=builder /bothan-api/registry /app/registry

WORKDIR /app
CMD ["bothan-api"]
