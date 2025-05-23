// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             (unknown)
// source: bothan/v1/bothan.proto

package proto

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	BothanService_GetInfo_FullMethodName               = "/bothan.v1.BothanService/GetInfo"
	BothanService_UpdateRegistry_FullMethodName        = "/bothan.v1.BothanService/UpdateRegistry"
	BothanService_PushMonitoringRecords_FullMethodName = "/bothan.v1.BothanService/PushMonitoringRecords"
	BothanService_GetPrices_FullMethodName             = "/bothan.v1.BothanService/GetPrices"
)

// BothanServiceClient is the client API for BothanService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type BothanServiceClient interface {
	// GetInfo gets information regarding Bothan's services
	GetInfo(ctx context.Context, in *GetInfoRequest, opts ...grpc.CallOption) (*GetInfoResponse, error)
	// UpdateRegistry updates the registry that Bothan uses with the given IPFS hash and version.
	UpdateRegistry(ctx context.Context, in *UpdateRegistryRequest, opts ...grpc.CallOption) (*UpdateRegistryResponse, error)
	// PushMonitoringRecords pushes the record associated with the given UUID to the monitoring service.
	// If monitoring is disabled in Bothan, this will always return an error.
	PushMonitoringRecords(ctx context.Context, in *PushMonitoringRecordsRequest, opts ...grpc.CallOption) (*PushMonitoringRecordsResponse, error)
	// GetPrices gets prices for the specified signal IDs.
	GetPrices(ctx context.Context, in *GetPricesRequest, opts ...grpc.CallOption) (*GetPricesResponse, error)
}

type bothanServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewBothanServiceClient(cc grpc.ClientConnInterface) BothanServiceClient {
	return &bothanServiceClient{cc}
}

func (c *bothanServiceClient) GetInfo(ctx context.Context, in *GetInfoRequest, opts ...grpc.CallOption) (*GetInfoResponse, error) {
	out := new(GetInfoResponse)
	err := c.cc.Invoke(ctx, BothanService_GetInfo_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *bothanServiceClient) UpdateRegistry(ctx context.Context, in *UpdateRegistryRequest, opts ...grpc.CallOption) (*UpdateRegistryResponse, error) {
	out := new(UpdateRegistryResponse)
	err := c.cc.Invoke(ctx, BothanService_UpdateRegistry_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *bothanServiceClient) PushMonitoringRecords(ctx context.Context, in *PushMonitoringRecordsRequest, opts ...grpc.CallOption) (*PushMonitoringRecordsResponse, error) {
	out := new(PushMonitoringRecordsResponse)
	err := c.cc.Invoke(ctx, BothanService_PushMonitoringRecords_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *bothanServiceClient) GetPrices(ctx context.Context, in *GetPricesRequest, opts ...grpc.CallOption) (*GetPricesResponse, error) {
	out := new(GetPricesResponse)
	err := c.cc.Invoke(ctx, BothanService_GetPrices_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// BothanServiceServer is the server API for BothanService service.
// All implementations must embed UnimplementedBothanServiceServer
// for forward compatibility
type BothanServiceServer interface {
	// GetInfo gets information regarding Bothan's services
	GetInfo(context.Context, *GetInfoRequest) (*GetInfoResponse, error)
	// UpdateRegistry updates the registry that Bothan uses with the given IPFS hash and version.
	UpdateRegistry(context.Context, *UpdateRegistryRequest) (*UpdateRegistryResponse, error)
	// PushMonitoringRecords pushes the record associated with the given UUID to the monitoring service.
	// If monitoring is disabled in Bothan, this will always return an error.
	PushMonitoringRecords(context.Context, *PushMonitoringRecordsRequest) (*PushMonitoringRecordsResponse, error)
	// GetPrices gets prices for the specified signal IDs.
	GetPrices(context.Context, *GetPricesRequest) (*GetPricesResponse, error)
	mustEmbedUnimplementedBothanServiceServer()
}

// UnimplementedBothanServiceServer must be embedded to have forward compatible implementations.
type UnimplementedBothanServiceServer struct {
}

func (UnimplementedBothanServiceServer) GetInfo(context.Context, *GetInfoRequest) (*GetInfoResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetInfo not implemented")
}
func (UnimplementedBothanServiceServer) UpdateRegistry(context.Context, *UpdateRegistryRequest) (*UpdateRegistryResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdateRegistry not implemented")
}
func (UnimplementedBothanServiceServer) PushMonitoringRecords(context.Context, *PushMonitoringRecordsRequest) (*PushMonitoringRecordsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PushMonitoringRecords not implemented")
}
func (UnimplementedBothanServiceServer) GetPrices(context.Context, *GetPricesRequest) (*GetPricesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPrices not implemented")
}
func (UnimplementedBothanServiceServer) mustEmbedUnimplementedBothanServiceServer() {}

// UnsafeBothanServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to BothanServiceServer will
// result in compilation errors.
type UnsafeBothanServiceServer interface {
	mustEmbedUnimplementedBothanServiceServer()
}

func RegisterBothanServiceServer(s grpc.ServiceRegistrar, srv BothanServiceServer) {
	s.RegisterService(&BothanService_ServiceDesc, srv)
}

func _BothanService_GetInfo_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetInfoRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(BothanServiceServer).GetInfo(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: BothanService_GetInfo_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(BothanServiceServer).GetInfo(ctx, req.(*GetInfoRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _BothanService_UpdateRegistry_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UpdateRegistryRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(BothanServiceServer).UpdateRegistry(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: BothanService_UpdateRegistry_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(BothanServiceServer).UpdateRegistry(ctx, req.(*UpdateRegistryRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _BothanService_PushMonitoringRecords_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PushMonitoringRecordsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(BothanServiceServer).PushMonitoringRecords(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: BothanService_PushMonitoringRecords_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(BothanServiceServer).PushMonitoringRecords(ctx, req.(*PushMonitoringRecordsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _BothanService_GetPrices_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetPricesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(BothanServiceServer).GetPrices(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: BothanService_GetPrices_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(BothanServiceServer).GetPrices(ctx, req.(*GetPricesRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// BothanService_ServiceDesc is the grpc.ServiceDesc for BothanService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var BothanService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "bothan.v1.BothanService",
	HandlerType: (*BothanServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetInfo",
			Handler:    _BothanService_GetInfo_Handler,
		},
		{
			MethodName: "UpdateRegistry",
			Handler:    _BothanService_UpdateRegistry_Handler,
		},
		{
			MethodName: "PushMonitoringRecords",
			Handler:    _BothanService_PushMonitoringRecords_Handler,
		},
		{
			MethodName: "GetPrices",
			Handler:    _BothanService_GetPrices_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "bothan/v1/bothan.proto",
}
