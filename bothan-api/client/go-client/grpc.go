package client

import (
	"context"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/price"
	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/signal"
)

var _ Client = &GrpcClient{}

type GrpcClient struct {
	connection *grpc.ClientConn
	timeout    time.Duration
}

func NewGrpcClient(url string, timeout time.Duration) (*GrpcClient, error) {
	connection, err := grpc.Dial(url, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, err
	}
	return &GrpcClient{connection, timeout}, nil
}

func (c *GrpcClient) UpdateRegistry(ipfsHash string, version string) error {
	client := signal.NewSignalServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	_, err := client.UpdateRegistry(ctx, &signal.UpdateRegistryRequest{IpfsHash: ipfsHash, Version: version})
	if err != nil {
		return err
	}

	return nil
}

func (c *GrpcClient) SetActiveSignalIDs(signalIDs []string) error {
	client := signal.NewSignalServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	_, err := client.SetActiveSignalIds(ctx, &signal.SetActiveSignalIdsRequest{SignalIds: signalIDs})
	if err != nil {
		return err
	}

	return nil
}

func (c *GrpcClient) GetPrices(signalIDs []string) ([]*price.Price, error) {
	// Create a client instance using the connection.
	client := price.NewPriceServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	response, err := client.GetPrices(ctx, &price.GetPricesRequest{SignalIds: signalIDs})
	if err != nil {
		return nil, err
	}

	return response.Prices, nil
}
