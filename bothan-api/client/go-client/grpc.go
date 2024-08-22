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

func (c *GRPC) QueryPrices(signalID string) ([]*proto.Price, error) {
	// Create a client instance using the connection.
	client := proto.NewQueryClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	response, err := client.GetPrices(ctx, &proto.QueryPricesRequest{SignalIDs: signalIds})
	if err != nil {
		return nil, err
	}

	return response.Prices, nil
}
