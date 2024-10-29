package client

import (
	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/price"
	"github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/signal"
)

type Client interface {
	GetInfo() (*signal.GetInfoResponse, error)
	UpdateRegistry(ipfsHash string, version string) error
	PushMonitoringRecords(uuid, txHash string) error
	GetPrices(signalIDs []string) (*price.GetPricesResponse, error)
}
