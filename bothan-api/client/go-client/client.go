package client

import (
	bothanproto "github.com/bandprotocol/bothan/bothan-api/client/go-client/query"
)

type Client interface {
	UpdateRegistry(ipfsHash string, version string) (*bothanproto.UpdateStatusCode, error)
	SetActiveSignalID(signalIDs []string) (bool, error)
	GetPrices(signalIDs []string) ([]*bothanproto.Price, error)
}
