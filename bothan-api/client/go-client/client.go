package client

import (
	proto "github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/price"
)

type Client interface {
	UpdateRegistry(ipfsHash string, version string) error
	SetActiveSignalIDs(signalIDs []string) error
	PushMonitoringRecords(uuid, txHash string) error
	GetPrices(signalIDs []string) (*proto.GetPricesResponse, error)
}
