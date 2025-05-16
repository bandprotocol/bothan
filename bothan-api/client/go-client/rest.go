package client

import (
	"encoding/json"
	"net/url"
	"path"
	"strings"
	"time"

	proto "github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/bothan/v1"
	"github.com/levigross/grequests"
)

var _ Client = &RestClient{}

type RestClient struct {
	url     string
	timeout time.Duration
}

func NewRestClient(url string, timeout time.Duration) *RestClient {
	return &RestClient{url, timeout}
}

func (c *RestClient) GetInfo() (*proto.GetInfoResponse, error) {
	parsedUrl, err := url.Parse(c.url + "/info")
	if err != nil {
		return nil, err
	}

	resp, err := grequests.Get(
		parsedUrl.String(),
		&grequests.RequestOptions{
			RequestTimeout: c.timeout,
		},
	)
	if err != nil {
		return nil, err
	}

	if !resp.Ok {
		return nil, resp.Error
	}

	var infoResp proto.GetInfoResponse
	err = json.Unmarshal(resp.Bytes(), &infoResp)
	if err != nil {
		return nil, err
	}

	return &infoResp, nil
}

func (c *RestClient) UpdateRegistry(ipfsHash string, version string) error {
	parsedUrl, err := url.Parse(c.url + "/registry")
	if err != nil {
		return err
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
		return err
	}

	if !resp.Ok {
		return resp.Error
	}

	return nil
}

func (c *RestClient) PushMonitoringRecords(uuid, txHash string, signalIDs []string) error {
	parsedUrl, err := url.Parse(c.url + "/monitoring_records")
	if err != nil {
		return err
	}

	resp, err := grequests.Post(
		parsedUrl.String(), &grequests.RequestOptions{
			RequestTimeout: c.timeout,
			JSON: map[string]any{
				"uuid":       uuid,
				"tx_hash":    txHash,
				"signal_ids": signalIDs,
			},
		},
	)

	if err != nil {
		return err
	}

	if !resp.Ok {
		return resp.Error
	}

	return nil
}

func (c *RestClient) GetPrices(signalIDs []string) (*proto.GetPricesResponse, error) {
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

	if !resp.Ok {
		return nil, resp.Error
	}

	var priceResp proto.GetPricesResponse
	err = json.Unmarshal(resp.Bytes(), &priceResp)
	if err != nil {
		return nil, err
	}

	return &priceResp, nil
}
