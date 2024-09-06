package main

import (
	"context"
	"fmt"
	"net/http"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"github.com/pelletier/go-toml"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/grpclog"

	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/price"
	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/signal"
)

type GrpcConfig struct {
	Addr string `toml:"addr"`
}

type GoProxyConfig struct {
	Addr string `toml:"addr"`
}

func run(grpcConfig GrpcConfig, goProxyConfig GoProxyConfig) error {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	// Register gRPC server endpoint
	// Note: Make sure the gRPC server is running properly and accessibly
	mux := runtime.NewServeMux()
	opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}

	err := signal.RegisterSignalServiceHandlerFromEndpoint(ctx, mux, grpcConfig.Addr, opts)
	if err != nil {
		return err
	}

	err = price.RegisterPriceServiceHandlerFromEndpoint(ctx, mux, grpcConfig.Addr, opts)
	if err != nil {
		return err
	}

	fmt.Println("Server running on", goProxyConfig.Addr)

	// Start an HTTP server (and proxy calls to gRPC server endpoint)
	return http.ListenAndServe(goProxyConfig.Addr, mux)
}

func main() {
	config, err := toml.LoadFile("./config.toml")
	if err != nil {
		fmt.Println("Error loading TOML file:", err)
		return
	}

	grpcTable, ok := config.Get("grpc").(*toml.Tree)
	if !ok {
		fmt.Println("gRPC configuration not found in TOML file")
		return
	}

	grpcConfig := GrpcConfig{}
	if err := grpcTable.Unmarshal(&grpcConfig); err != nil {
		fmt.Println("Error parsing gRPC config:", err)
		return
	}

	goProxyTable, ok := config.Get("go-proxy").(*toml.Tree)
	if !ok {
		fmt.Println("goProxy configuration not found in TOML file")
		return
	}

	goProxyConfig := GoProxyConfig{}
	if err := goProxyTable.Unmarshal(&goProxyConfig); err != nil {
		fmt.Println("Error parsing goProxy config:", err)
		return
	}

	if err := run(grpcConfig, goProxyConfig); err != nil {
		grpclog.Fatal(err)
	}
}
