package main

import (
	"context"
	"fmt"
	"net/http"
	"os"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/grpclog"

	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/price"
	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/signal"
)

func main() {
	grpcEndpoint := os.Getenv("GRPC_ENDPOINT")
	if grpcEndpoint == "" {
		grpcEndpoint = "localhost:50051"
	}

	proxyEndpoint := os.Getenv("PROXY_ENDPOINT")
	if proxyEndpoint == "" {
		proxyEndpoint = "localhost:8080"
	}

	if err := run(grpcEndpoint, proxyEndpoint); err != nil {
		grpclog.Fatal(err)
	}
}

func run(grpcEndpoint, proxyEndpoint string) error {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	// Register gRPC server endpoint
	// Note: Make sure the gRPC server is running properly and accessibly
	mux := runtime.NewServeMux()
	opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}

	err := signal.RegisterSignalServiceHandlerFromEndpoint(ctx, mux, grpcEndpoint, opts)
	if err != nil {
		return err
	}

	err = price.RegisterPriceServiceHandlerFromEndpoint(ctx, mux, grpcEndpoint, opts)
	if err != nil {
		return err
	}

	fmt.Println("Server running on", proxyEndpoint)

	// Start an HTTP server (and proxy calls to gRPC server endpoint)
	return http.ListenAndServe(proxyEndpoint, mux)
}
