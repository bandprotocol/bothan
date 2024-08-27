package client

import (
	"context"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	proto "github.com/bandprotocol/bothan/bothan-api/client/go-client/query"
)

var _ Client = &GRPC{}

type GRPC struct {
	connection *grpc.ClientConn
	timeout    time.Duration
}

func NewGRPC(url string, timeout time.Duration) (*GRPC, error) {
	connection, err := grpc.Dial(url, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, err
	}
	return &GRPC{connection, timeout}, nil
}

func (c *GRPC) UpdateRegistry(ipfsHash string, version string) (*proto.UpdateStatusCode, error) {
	client := proto.NewQueryClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	response, err := client.UpdateRegistry(ctx, &proto.UpdateRegistryRequest{IpfsHash: ipfsHash, Version: version})
	if err != nil {
		return nil, err
	}

	return &response.Code, nil
}

func (c *GRPC) SetActiveSignalIDs(signalIDs []string) (bool, error) {
	client := proto.NewQueryClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	response, err := client.SetActiveSignalID(ctx, &proto.SetActiveSignalIDRequest{SignalIds: signalIDs})
	if err != nil {
		return false, err
	}

	return response.Success, nil
}

func (c *GRPC) GetPrices(signalIDs []string) ([]*proto.Price, error) {
	// Create a client instance using the connection.
	client := proto.NewQueryClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	response, err := client.GetPrices(ctx, &proto.PriceRequest{SignalIds: signalIDs})
	if err != nil {
		return nil, err
	}

	return response.Prices, nil
}
