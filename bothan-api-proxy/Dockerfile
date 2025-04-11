FROM golang:1.22.2-bookworm AS builder

WORKDIR /build

COPY bothan-api/client/go-client/ bothan-api/client/go-client/
COPY bothan-api-proxy/ bothan-api-proxy/

WORKDIR /build/bothan-api-proxy

RUN go mod download
RUN go build -o app

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /build/bothan-api-proxy/app .

CMD ["./app"]
