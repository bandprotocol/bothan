module go-proxy

go 1.22.0

require (
	github.com/bandprotocol/bothan/bothan-api/client/go-client v0.0.1
	github.com/grpc-ecosystem/grpc-gateway/v2 v2.22.0
	google.golang.org/grpc v1.67.1
)

require (
	golang.org/x/net v0.28.0 // indirect
	golang.org/x/sys v0.24.0 // indirect
	golang.org/x/text v0.17.0 // indirect
	google.golang.org/genproto/googleapis/api v0.0.0-20241021214115-324edc3d5d38 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20241015192408-796eee8c2d53 // indirect
	google.golang.org/protobuf v1.35.1 // indirect
)

replace github.com/bandprotocol/bothan/bothan-api/client/go-client => ../bothan-api/client/go-client
