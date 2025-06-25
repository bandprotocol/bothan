// Package client provides a Go client for interacting with the Bothan API Server.
//
// This package offers both gRPC and REST client implementations for accessing
// cryptocurrency data and managing the Bothan ecosystem.
//
// # Overview
//
// The Bothan API Go Client provides:
//   - High-level client abstractions for API interactions
//   - Protocol buffer definitions and generated code
//   - Support for both gRPC and REST endpoints
//   - Comprehensive error handling and type safety
//
// # Features
//
//   - **gRPC Client**: High-performance binary protocol client
//   - **REST Client**: HTTP-based REST API client
//   - **Protocol Buffers**: Type-safe message definitions
//   - **Error Handling**: Comprehensive error types and handling
//
// # Usage
//
//	package main
//
//	import (
//		"fmt"
//		"log"
//
//		"github.com/bandprotocol/bothan/bothan-api/client/go-client"
//	)
//
//	func main() {
//		// Create a new client
//		c := client.NewRestClient("http://localhost:8080")
//
//		// Get server information
//		info, err := c.GetInfo()
//		if err != nil {
//			log.Fatal(err)
//		}
//		fmt.Printf("Server version: %s\n", info.Version)
//
//		// Get prices for specific signal IDs
//		prices, err := c.GetPrices([]string{"BTC/USD", "ETH/USD"})
//		if err != nil {
//			log.Fatal(err)
//		}
//		for _, price := range prices.Prices {
//			fmt.Printf("%s: %f\n", price.SignalId, price.Price)
//		}
//	}
//
// # API Methods
//
// The Client interface provides the following methods:
//   - GetInfo(): Retrieve server information and status
//   - UpdateRegistry(): Update the data registry with new IPFS hash
//   - PushMonitoringRecords(): Submit monitoring records for tracking
//   - GetPrices(): Retrieve current prices for specified signal IDs
package client

import proto "github.com/bandprotocol/bothan/bothan-api/client/go-client/proto/bothan/v1"

// Client defines the interface for interacting with the Bothan API Server.
//
// This interface provides methods for:
//   - Retrieving server information and status
//   - Updating the data registry with new data sources
//   - Submitting monitoring records for tracking and validation
//   - Fetching current cryptocurrency prices and market data
//
// The interface is implemented by both gRPC and REST clients,
// allowing for protocol-agnostic usage patterns.
type Client interface {
	// GetInfo retrieves information about the Bothan API Server.
	//
	// Returns server version, status, and configuration information
	// that can be used for health checks and client compatibility.
	GetInfo() (*proto.GetInfoResponse, error)

	// UpdateRegistry updates the data registry with a new IPFS hash and version.
	//
	// This method is used to publish new data sources or updates
	// to the Bothan ecosystem. The IPFS hash should point to
	// a valid data source that conforms to the Bothan protocol.
	//
	// Parameters:
	//   - ipfsHash: The IPFS hash of the new data source
	//   - version: The version string for the update
	UpdateRegistry(ipfsHash string, version string) error

	// PushMonitoringRecords submits monitoring records for tracking and validation.
	//
	// This method allows clients to submit records that will be
	// monitored for accuracy and performance. The records are
	// associated with a UUID and transaction hash for tracking.
	//
	// Parameters:
	//   - uuid: Unique identifier for the monitoring session
	//   - txHash: Transaction hash associated with the records
	//   - signalIDs: List of signal IDs being monitored
	PushMonitoringRecords(uuid, txHash string, signalIDs []string) error

	// GetPrices retrieves current prices for the specified signal IDs.
	//
	// Returns the latest price data for the requested cryptocurrency
	// pairs or signal identifiers. The response includes price,
	// timestamp, and metadata for each requested signal.
	//
	// Parameters:
	//   - signalIDs: List of signal IDs to retrieve prices for
	//
	// Returns a response containing price data for each signal ID
	GetPrices(signalIDs []string) (*proto.GetPricesResponse, error)
}
