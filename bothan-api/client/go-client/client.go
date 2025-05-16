package client

import (
	proto "github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/bothan/v1"
)

type Client interface {
	GetInfo() (*proto.GetInfoResponse, error)
	UpdateRegistry(ipfsHash string, version string) error
	PushMonitoringRecords(uuid, txHash string, signalIDs []string) error
	GetPrices(signalIDs []string) (*proto.GetPricesResponse, error)
}
