// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.0
// 	protoc        (unknown)
// source: query/query.proto

package query

import (
	_ "google.golang.org/genproto/googleapis/api/annotations"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// PriceOption defines the price option of a price.
type Status int32

const (
	// PRICE_STATUS_UNSUPPORTED defines an unsupported price status.
	Status_PRICE_STATUS_UNSUPPORTED Status = 0
	// PRICE_STATUS_UNAVAILABLE defines an unavailable price status.
	Status_PRICE_STATUS_UNAVAILABLE Status = 1
	// PRICE_STATUS_AVAILABLE defines an available price status.
	Status_PRICE_STATUS_AVAILABLE Status = 2
)

// Enum value maps for Status.
var (
	Status_name = map[int32]string{
		0: "PRICE_STATUS_UNSUPPORTED",
		1: "PRICE_STATUS_UNAVAILABLE",
		2: "PRICE_STATUS_AVAILABLE",
	}
	Status_value = map[string]int32{
		"PRICE_STATUS_UNSUPPORTED": 0,
		"PRICE_STATUS_UNAVAILABLE": 1,
		"PRICE_STATUS_AVAILABLE":   2,
	}
)

func (x Status) Enum() *Status {
	p := new(Status)
	*p = x
	return p
}

func (x Status) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (Status) Descriptor() protoreflect.EnumDescriptor {
	return file_query_query_proto_enumTypes[0].Descriptor()
}

func (Status) Type() protoreflect.EnumType {
	return &file_query_query_proto_enumTypes[0]
}

func (x Status) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use Status.Descriptor instead.
func (Status) EnumDescriptor() ([]byte, []int) {
	return file_query_query_proto_rawDescGZIP(), []int{0}
}

// QueryPricesRequest is the request type for the PriceService/GetPrices RPC
// method.
type QueryPricesRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	SignalIds []string `protobuf:"bytes,1,rep,name=signal_ids,json=signalIds,proto3" json:"signal_ids,omitempty"`
}

func (x *QueryPricesRequest) Reset() {
	*x = QueryPricesRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_query_query_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *QueryPricesRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QueryPricesRequest) ProtoMessage() {}

func (x *QueryPricesRequest) ProtoReflect() protoreflect.Message {
	mi := &file_query_query_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use QueryPricesRequest.ProtoReflect.Descriptor instead.
func (*QueryPricesRequest) Descriptor() ([]byte, []int) {
	return file_query_query_proto_rawDescGZIP(), []int{0}
}

func (x *QueryPricesRequest) GetSignalIds() []string {
	if x != nil {
		return x.SignalIds
	}
	return nil
}

// QueryPricesResponse is the response type for the PriceService/GetPrices RPC
// method.
type QueryPricesResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Prices []*AssetPrice `protobuf:"bytes,1,rep,name=prices,proto3" json:"prices,omitempty"`
}

func (x *QueryPricesResponse) Reset() {
	*x = QueryPricesResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_query_query_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *QueryPricesResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QueryPricesResponse) ProtoMessage() {}

func (x *QueryPricesResponse) ProtoReflect() protoreflect.Message {
	mi := &file_query_query_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use QueryPricesResponse.ProtoReflect.Descriptor instead.
func (*QueryPricesResponse) Descriptor() ([]byte, []int) {
	return file_query_query_proto_rawDescGZIP(), []int{1}
}

func (x *QueryPricesResponse) GetPrices() []*AssetPrice {
	if x != nil {
		return x.Prices
	}
	return nil
}

// PriceData defines the data of a symbol price.
type AssetPrice struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The symbol of the price.
	SignalId string `protobuf:"bytes,1,opt,name=signal_id,json=signalId,proto3" json:"signal_id,omitempty"`
	// The price of the symbol.
	Price string `protobuf:"bytes,2,opt,name=price,proto3" json:"price,omitempty"`
	// PriceStatus defines the price status of a symbol.
	Status Status `protobuf:"varint,3,opt,name=status,proto3,enum=query.Status" json:"status,omitempty"`
}

func (x *AssetPrice) Reset() {
	*x = AssetPrice{}
	if protoimpl.UnsafeEnabled {
		mi := &file_query_query_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AssetPrice) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AssetPrice) ProtoMessage() {}

func (x *AssetPrice) ProtoReflect() protoreflect.Message {
	mi := &file_query_query_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AssetPrice.ProtoReflect.Descriptor instead.
func (*AssetPrice) Descriptor() ([]byte, []int) {
	return file_query_query_proto_rawDescGZIP(), []int{2}
}

func (x *AssetPrice) GetSignalId() string {
	if x != nil {
		return x.SignalId
	}
	return ""
}

func (x *AssetPrice) GetPrice() string {
	if x != nil {
		return x.Price
	}
	return ""
}

func (x *AssetPrice) GetStatus() Status {
	if x != nil {
		return x.Status
	}
	return Status_PRICE_STATUS_UNSUPPORTED
}

var File_query_query_proto protoreflect.FileDescriptor

var file_query_query_proto_rawDesc = []byte{
	0x0a, 0x11, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x12, 0x05, 0x71, 0x75, 0x65, 0x72, 0x79, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x33, 0x0a, 0x12, 0x51, 0x75, 0x65, 0x72,
	0x79, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d,
	0x0a, 0x0a, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03,
	0x28, 0x09, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x73, 0x22, 0x40, 0x0a,
	0x13, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70,
	0x6f, 0x6e, 0x73, 0x65, 0x12, 0x29, 0x0a, 0x06, 0x70, 0x72, 0x69, 0x63, 0x65, 0x73, 0x18, 0x01,
	0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x41, 0x73, 0x73,
	0x65, 0x74, 0x50, 0x72, 0x69, 0x63, 0x65, 0x52, 0x06, 0x70, 0x72, 0x69, 0x63, 0x65, 0x73, 0x22,
	0x66, 0x0a, 0x0a, 0x41, 0x73, 0x73, 0x65, 0x74, 0x50, 0x72, 0x69, 0x63, 0x65, 0x12, 0x1b, 0x0a,
	0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x08, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x72,
	0x69, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x72, 0x69, 0x63, 0x65,
	0x12, 0x25, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e,
	0x32, 0x0d, 0x2e, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52,
	0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2a, 0x60, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75,
	0x73, 0x12, 0x1c, 0x0a, 0x18, 0x50, 0x52, 0x49, 0x43, 0x45, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55,
	0x53, 0x5f, 0x55, 0x4e, 0x53, 0x55, 0x50, 0x50, 0x4f, 0x52, 0x54, 0x45, 0x44, 0x10, 0x00, 0x12,
	0x1c, 0x0a, 0x18, 0x50, 0x52, 0x49, 0x43, 0x45, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f,
	0x55, 0x4e, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x01, 0x12, 0x1a, 0x0a,
	0x16, 0x50, 0x52, 0x49, 0x43, 0x45, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x41, 0x56,
	0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x02, 0x32, 0x66, 0x0a, 0x05, 0x51, 0x75, 0x65,
	0x72, 0x79, 0x12, 0x5d, 0x0a, 0x06, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x12, 0x19, 0x2e, 0x71,
	0x75, 0x65, 0x72, 0x79, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1a, 0x2e, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e,
	0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x22, 0x1c, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x16, 0x12, 0x14, 0x2f, 0x70, 0x72,
	0x69, 0x63, 0x65, 0x73, 0x2f, 0x7b, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x73,
	0x7d, 0x42, 0x12, 0x5a, 0x10, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2d, 0x61, 0x70, 0x69, 0x2f,
	0x71, 0x75, 0x65, 0x72, 0x79, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_query_query_proto_rawDescOnce sync.Once
	file_query_query_proto_rawDescData = file_query_query_proto_rawDesc
)

func file_query_query_proto_rawDescGZIP() []byte {
	file_query_query_proto_rawDescOnce.Do(func() {
		file_query_query_proto_rawDescData = protoimpl.X.CompressGZIP(file_query_query_proto_rawDescData)
	})
	return file_query_query_proto_rawDescData
}

var file_query_query_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_query_query_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_query_query_proto_goTypes = []interface{}{
	(Status)(0),                 // 0: query.Status
	(*QueryPricesRequest)(nil),  // 1: query.QueryPricesRequest
	(*QueryPricesResponse)(nil), // 2: query.QueryPricesResponse
	(*AssetPrice)(nil),          // 3: query.AssetPrice
}
var file_query_query_proto_depIdxs = []int32{
	3, // 0: query.QueryPricesResponse.prices:type_name -> query.AssetPrice
	0, // 1: query.AssetPrice.status:type_name -> query.Status
	1, // 2: query.Query.Prices:input_type -> query.QueryPricesRequest
	2, // 3: query.Query.Prices:output_type -> query.QueryPricesResponse
	3, // [3:4] is the sub-list for method output_type
	2, // [2:3] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_query_query_proto_init() }
func file_query_query_proto_init() {
	if File_query_query_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_query_query_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*QueryPricesRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_query_query_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*QueryPricesResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_query_query_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AssetPrice); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_query_query_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_query_query_proto_goTypes,
		DependencyIndexes: file_query_query_proto_depIdxs,
		EnumInfos:         file_query_query_proto_enumTypes,
		MessageInfos:      file_query_query_proto_msgTypes,
	}.Build()
	File_query_query_proto = out.File
	file_query_query_proto_rawDesc = nil
	file_query_query_proto_goTypes = nil
	file_query_query_proto_depIdxs = nil
}
