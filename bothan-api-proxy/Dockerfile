FROM golang:1.22.2-bookworm

WORKDIR /app

COPY /bothan-api-proxy/ ./
RUN go mod download

RUN go build -o main .

CMD ["./main"]
