package client

import (
	"context"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/bothan/v1"
)

var _ Client = &GrpcClient{}

type GrpcClient struct {
	connection *grpc.ClientConn
	timeout    time.Duration
}

func NewGrpcClient(url string, timeout time.Duration) (*GrpcClient, error) {
	connection, err := grpc.NewClient(url, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, err
	}
	return &GrpcClient{connection, timeout}, nil
}

func (c *GrpcClient) GetInfo() (*proto.GetInfoResponse, error) {
	client := proto.NewBothanServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	return client.GetInfo(ctx, &proto.GetInfoRequest{})
}

func (c *GrpcClient) UpdateRegistry(ipfsHash string, version string) error {
	client := proto.NewBothanServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	_, err := client.UpdateRegistry(ctx, &proto.UpdateRegistryRequest{IpfsHash: ipfsHash, Version: version})
	return err
}

func (c *GrpcClient) PushMonitoringRecords(uuid, txHash string) error {
	client := proto.NewBothanServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	_, err := client.PushMonitoringRecords(ctx, &proto.PushMonitoringRecordsRequest{Uuid: uuid, TxHash: txHash})
	return err
}

func (c *GrpcClient) GetPrices(signalIDs []string) (*proto.GetPricesResponse, error) {
	// Create a client instance using the connection.
	client := proto.NewBothanServiceClient(c.connection)
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()

	return client.GetPrices(ctx, &proto.GetPricesRequest{SignalIds: signalIDs})
}
