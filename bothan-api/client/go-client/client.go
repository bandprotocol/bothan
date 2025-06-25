package client

import proto "github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/bothan/v1"

// Client defines the interface for interacting with the Bothan API client.
// It provides methods to retrieve information, update the registry, push monitoring records,
// and fetch prices for given signal IDs.
type Client interface {
	GetInfo() (*proto.GetInfoResponse, error)
	UpdateRegistry(ipfsHash string, version string) error
	PushMonitoringRecords(uuid, txHash string, signalIDs []string) error
	GetPrices(signalIDs []string) (*proto.GetPricesResponse, error)
}
