// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             (unknown)
// source: query/query.proto

package query

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
	Query_UpdateRegistry_FullMethodName    = "/query.Query/UpdateRegistry"
	Query_SetActiveSignalID_FullMethodName = "/query.Query/SetActiveSignalID"
	Query_GetPrice_FullMethodName          = "/query.Query/GetPrice"
)

// QueryClient is the client API for Query service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type QueryClient interface {
	UpdateRegistry(ctx context.Context, in *UpdateRegistryRequest, opts ...grpc.CallOption) (*UpdateRegistryResponse, error)
	SetActiveSignalID(ctx context.Context, in *SetActiveSignalIDRequest, opts ...grpc.CallOption) (*SetActiveSignalIDResponse, error)
	// RPC method that returns all prices of requested signal ids.
	GetPrice(ctx context.Context, in *PriceRequest, opts ...grpc.CallOption) (*PriceResponse, error)
}

type queryClient struct {
	cc grpc.ClientConnInterface
}

func NewQueryClient(cc grpc.ClientConnInterface) QueryClient {
	return &queryClient{cc}
}

func (c *queryClient) UpdateRegistry(ctx context.Context, in *UpdateRegistryRequest, opts ...grpc.CallOption) (*UpdateRegistryResponse, error) {
	out := new(UpdateRegistryResponse)
	err := c.cc.Invoke(ctx, Query_UpdateRegistry_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *queryClient) SetActiveSignalID(ctx context.Context, in *SetActiveSignalIDRequest, opts ...grpc.CallOption) (*SetActiveSignalIDResponse, error) {
	out := new(SetActiveSignalIDResponse)
	err := c.cc.Invoke(ctx, Query_SetActiveSignalID_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *queryClient) GetPrice(ctx context.Context, in *PriceRequest, opts ...grpc.CallOption) (*PriceResponse, error) {
	out := new(PriceResponse)
	err := c.cc.Invoke(ctx, Query_GetPrice_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// QueryServer is the server API for Query service.
// All implementations must embed UnimplementedQueryServer
// for forward compatibility
type QueryServer interface {
	UpdateRegistry(context.Context, *UpdateRegistryRequest) (*UpdateRegistryResponse, error)
	SetActiveSignalID(context.Context, *SetActiveSignalIDRequest) (*SetActiveSignalIDResponse, error)
	// RPC method that returns all prices of requested signal ids.
	GetPrice(context.Context, *PriceRequest) (*PriceResponse, error)
	mustEmbedUnimplementedQueryServer()
}

// UnimplementedQueryServer must be embedded to have forward compatible implementations.
type UnimplementedQueryServer struct {
}

func (UnimplementedQueryServer) UpdateRegistry(context.Context, *UpdateRegistryRequest) (*UpdateRegistryResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdateRegistry not implemented")
}
func (UnimplementedQueryServer) SetActiveSignalID(context.Context, *SetActiveSignalIDRequest) (*SetActiveSignalIDResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SetActiveSignalID not implemented")
}
func (UnimplementedQueryServer) GetPrice(context.Context, *PriceRequest) (*PriceResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPrice not implemented")
}
func (UnimplementedQueryServer) mustEmbedUnimplementedQueryServer() {}

// UnsafeQueryServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to QueryServer will
// result in compilation errors.
type UnsafeQueryServer interface {
	mustEmbedUnimplementedQueryServer()
}

func RegisterQueryServer(s grpc.ServiceRegistrar, srv QueryServer) {
	s.RegisterService(&Query_ServiceDesc, srv)
}

func _Query_UpdateRegistry_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UpdateRegistryRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QueryServer).UpdateRegistry(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Query_UpdateRegistry_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QueryServer).UpdateRegistry(ctx, req.(*UpdateRegistryRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Query_SetActiveSignalID_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SetActiveSignalIDRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QueryServer).SetActiveSignalID(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Query_SetActiveSignalID_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QueryServer).SetActiveSignalID(ctx, req.(*SetActiveSignalIDRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Query_GetPrice_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PriceRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QueryServer).GetPrice(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Query_GetPrice_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QueryServer).GetPrice(ctx, req.(*PriceRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Query_ServiceDesc is the grpc.ServiceDesc for Query service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Query_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "query.Query",
	HandlerType: (*QueryServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "UpdateRegistry",
			Handler:    _Query_UpdateRegistry_Handler,
		},
		{
			MethodName: "SetActiveSignalID",
			Handler:    _Query_SetActiveSignalID_Handler,
		},
		{
			MethodName: "GetPrice",
			Handler:    _Query_GetPrice_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "query/query.proto",
}
