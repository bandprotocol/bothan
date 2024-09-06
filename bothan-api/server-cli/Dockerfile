# Start from the latest Rust image
FROM rust:1.78-bookworm AS builder

WORKDIR /builder
# Copy entire source code
COPY . .

# Build the project
RUN apt-get update && apt-get -y install clang
RUN cargo build -p bothan-api-cli --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*.
RUN ldconfig

# Copy the binary from builder
COPY --from=builder builder/target/release/bothan /usr/local/bin

WORKDIR /app
CMD ["bothan", "start"]
