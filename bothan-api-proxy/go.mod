module go-proxy

go 1.22.0

require (
	github.com/bandprotocol/bothan/bothan-api/client v0.0.1
	github.com/grpc-ecosystem/grpc-gateway/v2 v2.19.1
	github.com/pelletier/go-toml v1.9.5
	google.golang.org/grpc v1.63.2
)

require (
	golang.org/x/net v0.24.0 // indirect
	golang.org/x/sys v0.19.0 // indirect
	golang.org/x/text v0.14.0 // indirect
	google.golang.org/genproto/googleapis/api v0.0.0-20240506185236-b8a5c65736ae // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20240429193739-8cf5692501f6 // indirect
	google.golang.org/protobuf v1.34.1 // indirect
)

replace github.com/bandprotocol/bothan/bothan-api/client => ../bothan-api/client/go-client
