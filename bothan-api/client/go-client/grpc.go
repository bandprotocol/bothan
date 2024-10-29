package client

import (
	"context"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/protobuf/types/known/emptypb"

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

func (c *GrpcClient) GetInfo() (*signal.GetInfoResponse, error) {
	client := signal.NewSignalServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	return client.GetInfo(ctx, &emptypb.Empty{})
}

func (c *GrpcClient) UpdateRegistry(ipfsHash string, version string) error {
	client := signal.NewSignalServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	_, err := client.UpdateRegistry(ctx, &signal.UpdateRegistryRequest{IpfsHash: ipfsHash, Version: version})
	return err
}

func (c *GrpcClient) PushMonitoringRecords(uuid, txHash string) error {
	client := signal.NewSignalServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	_, err := client.PushMonitoringRecords(ctx, &signal.PushMonitoringRecordsRequest{Uuid: uuid, TxHash: txHash})
	return err
}

func (c *GrpcClient) GetPrices(signalIDs []string) (*price.GetPricesResponse, error) {
	// Create a client instance using the connection.
	client := price.NewPriceServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	return client.GetPrices(ctx, &price.GetPricesRequest{SignalIds: signalIDs})
}
