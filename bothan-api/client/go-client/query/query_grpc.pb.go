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
	Query_UpdateRegistry_FullMethodName     = "/query.Query/UpdateRegistry"
	Query_SetActiveSignalIds_FullMethodName = "/query.Query/SetActiveSignalIds"
	Query_GetPrices_FullMethodName          = "/query.Query/GetPrices"
)

// QueryClient is the client API for Query service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type QueryClient interface {
	UpdateRegistry(ctx context.Context, in *UpdateRegistryRequest, opts ...grpc.CallOption) (*UpdateRegistryResponse, error)
	SetActiveSignalIds(ctx context.Context, in *SetActiveSignalIdsRequest, opts ...grpc.CallOption) (*SetActiveSignalIdsResponse, error)
	// RPC method that returns all prices of requested signal ids.
	GetPrices(ctx context.Context, in *GetPricesRequest, opts ...grpc.CallOption) (*GetPricesResponse, error)
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

func (c *queryClient) SetActiveSignalIds(ctx context.Context, in *SetActiveSignalIdsRequest, opts ...grpc.CallOption) (*SetActiveSignalIdsResponse, error) {
	out := new(SetActiveSignalIdsResponse)
	err := c.cc.Invoke(ctx, Query_SetActiveSignalIds_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *queryClient) GetPrices(ctx context.Context, in *GetPricesRequest, opts ...grpc.CallOption) (*GetPricesResponse, error) {
	out := new(GetPricesResponse)
	err := c.cc.Invoke(ctx, Query_GetPrices_FullMethodName, in, out, opts...)
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
	SetActiveSignalIds(context.Context, *SetActiveSignalIdsRequest) (*SetActiveSignalIdsResponse, error)
	// RPC method that returns all prices of requested signal ids.
	GetPrices(context.Context, *GetPricesRequest) (*GetPricesResponse, error)
	mustEmbedUnimplementedQueryServer()
}

// UnimplementedQueryServer must be embedded to have forward compatible implementations.
type UnimplementedQueryServer struct {
}

func (UnimplementedQueryServer) UpdateRegistry(context.Context, *UpdateRegistryRequest) (*UpdateRegistryResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdateRegistry not implemented")
}
func (UnimplementedQueryServer) SetActiveSignalIds(context.Context, *SetActiveSignalIdsRequest) (*SetActiveSignalIdsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SetActiveSignalIds not implemented")
}
func (UnimplementedQueryServer) GetPrices(context.Context, *GetPricesRequest) (*GetPricesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPrices not implemented")
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

func _Query_SetActiveSignalIds_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SetActiveSignalIdsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QueryServer).SetActiveSignalIds(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Query_SetActiveSignalIds_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QueryServer).SetActiveSignalIds(ctx, req.(*SetActiveSignalIdsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Query_GetPrices_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetPricesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QueryServer).GetPrices(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Query_GetPrices_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QueryServer).GetPrices(ctx, req.(*GetPricesRequest))
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
			MethodName: "SetActiveSignalIds",
			Handler:    _Query_SetActiveSignalIds_Handler,
		},
		{
			MethodName: "GetPrices",
			Handler:    _Query_GetPrices_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "query/query.proto",
}
