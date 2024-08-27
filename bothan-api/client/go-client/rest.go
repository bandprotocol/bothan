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

func (c *RestClient) UpdateRegistry(ipfsHash string, version string) (*proto.UpdateStatusCode, error) {
	parsedUrl, err := url.Parse(c.url + "/registry")
	if err != nil {
		return nil, err
	}

	resp, err := grequests.Post(
		parsedUrl.String(), &grequests.RequestOptions{
			RequestTimeout: c.timeout,
			JSON: map[string]string{
				"ipfsHash": ipfsHash,
				"version":  version,
			},
		},
	)

	if err != nil {
		return nil, err
	}

	var updateResp proto.UpdateStatusCode
	err = json.Unmarshal(resp.Bytes(), &updateResp)
	if err != nil {
		return nil, err
	}

	return &updateResp, nil
}

func (c *RestClient) SetActiveSignalIDs(signalIDs []string) (bool, error) {
	parsedUrl, err := url.Parse(c.url + "/signal_ids")
	if err != nil {
		return false, err
	}

	resp, err := grequests.Post(
		parsedUrl.String(), &grequests.RequestOptions{
			RequestTimeout: c.timeout,
			JSON: map[string][]string{
				"signal_ids": signalIDs,
			},
		},
	)

	if err != nil {
		return false, err
	}

	var setActiveResp proto.SetActiveSignalIDResponse
	err = json.Unmarshal(resp.Bytes(), &setActiveResp)
	if err != nil {
		return false, err
	}

	return setActiveResp.Success, nil
}

func (c *RestClient) GetPrices(signalIDs []string) ([]*proto.Price, error) {
	parsedUrl, err := url.Parse(c.url + "/prices")
	if err != nil {
		return nil, err
	}
	parsedUrl.Path = path.Join(parsedUrl.Path, strings.Join(signalIDs, ","))

	resp, err := grequests.Get(
		parsedUrl.String(),
		&grequests.RequestOptions{
			RequestTimeout: c.timeout,
		},
	)
	if err != nil {
		return nil, err
	}

	var priceResp proto.PriceResponse
	err = json.Unmarshal(resp.Bytes(), &priceResp)
	if err != nil {
		return nil, err
	}

	return priceResp.Prices, nil
}
