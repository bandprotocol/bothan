package client

import (
	bothanproto "github.com/bandprotocol/bothan/bothan-api/client/go-client/query"
)

type Client interface {
	QueryPrices(signalIDs []string) ([]*bothanproto.PriceData, error)
}
