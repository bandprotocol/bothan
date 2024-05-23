package client

import (
	"encoding/json"
	"net/url"
	"path"
	"strings"
	"time"

	"github.com/levigross/grequests"

	proto "github.com/bandprotocol/bothan/bothan-api/client/go-client/query"
)

var _ Client = &RestClient{}

type RestClient struct {
	url     string
	timeout time.Duration
}

func NewRest(url string, timeout time.Duration) *RestClient {
	return &RestClient{url, timeout}
}

func (c *RestClient) QueryPrices(signalIds []string) ([]*proto.PriceData, error) {
	parsedUrl, err := url.Parse(c.url + "/prices")
	if err != nil {
		return nil, err
	}
	parsedUrl.Path = path.Join(parsedUrl.Path, strings.Join(signalIds, ","))

	resp, err := grequests.Get(
		parsedUrl.String(),
		&grequests.RequestOptions{
			RequestTimeout: c.timeout,
		},
	)
	if err != nil {
		return nil, err
	}

	var priceResp proto.QueryPricesResponse
	err = json.Unmarshal(resp.Bytes(), &priceResp)
	if err != nil {
		return nil, err
	}

	return priceResp.Prices, nil
}
