// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.32.0
// 	protoc        (unknown)
// source: proto/query.proto

package query

import (
	_ "github.com/cosmos/gogoproto/gogoproto"
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
type PriceOption int32

const (
	// PRICE_OPTION_UNSPECIFIED defines an unspecified price option.
	PriceOption_PRICE_OPTION_UNSPECIFIED PriceOption = 0
	// PRICE_OPTION_UNSUPPORTED defines an unsupported price option.
	PriceOption_PRICE_OPTION_UNSUPPORTED PriceOption = 1
	// PRICE_OPTION_UNAVAILABLE defines an unavailable price option.
	PriceOption_PRICE_OPTION_UNAVAILABLE PriceOption = 2
	// PRICE_OPTION_AVAILABLE defines an available price option.
	PriceOption_PRICE_OPTION_AVAILABLE PriceOption = 3
)

// Enum value maps for PriceOption.
var (
	PriceOption_name = map[int32]string{
		0: "PRICE_OPTION_UNSPECIFIED",
		1: "PRICE_OPTION_UNSUPPORTED",
		2: "PRICE_OPTION_UNAVAILABLE",
		3: "PRICE_OPTION_AVAILABLE",
	}
	PriceOption_value = map[string]int32{
		"PRICE_OPTION_UNSPECIFIED": 0,
		"PRICE_OPTION_UNSUPPORTED": 1,
		"PRICE_OPTION_UNAVAILABLE": 2,
		"PRICE_OPTION_AVAILABLE":   3,
	}
)

func (x PriceOption) Enum() *PriceOption {
	p := new(PriceOption)
	*p = x
	return p
}

func (x PriceOption) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (PriceOption) Descriptor() protoreflect.EnumDescriptor {
	return file_proto_query_proto_enumTypes[0].Descriptor()
}

func (PriceOption) Type() protoreflect.EnumType {
	return &file_proto_query_proto_enumTypes[0]
}

func (x PriceOption) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use PriceOption.Descriptor instead.
func (PriceOption) EnumDescriptor() ([]byte, []int) {
	return file_proto_query_proto_rawDescGZIP(), []int{0}
}

// QueryPricesRequest is the request type for the PriceService/GetPrices RPC
// method.
type QueryPricesRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Symbols []string `protobuf:"bytes,1,rep,name=symbols,proto3" json:"symbols,omitempty"`
}

func (x *QueryPricesRequest) Reset() {
	*x = QueryPricesRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_query_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *QueryPricesRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QueryPricesRequest) ProtoMessage() {}

func (x *QueryPricesRequest) ProtoReflect() protoreflect.Message {
	mi := &file_proto_query_proto_msgTypes[0]
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
	return file_proto_query_proto_rawDescGZIP(), []int{0}
}

func (x *QueryPricesRequest) GetSymbols() []string {
	if x != nil {
		return x.Symbols
	}
	return nil
}

// QueryPricesResponse is the response type for the PriceService/GetPrices RPC
// method.
type QueryPricesResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Prices []*PriceData `protobuf:"bytes,1,rep,name=prices,proto3" json:"prices,omitempty"`
}

func (x *QueryPricesResponse) Reset() {
	*x = QueryPricesResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_query_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *QueryPricesResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QueryPricesResponse) ProtoMessage() {}

func (x *QueryPricesResponse) ProtoReflect() protoreflect.Message {
	mi := &file_proto_query_proto_msgTypes[1]
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
	return file_proto_query_proto_rawDescGZIP(), []int{1}
}

func (x *QueryPricesResponse) GetPrices() []*PriceData {
	if x != nil {
		return x.Prices
	}
	return nil
}

// PriceData defines the data of a symbol price.
type PriceData struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The symbol of the price.
	Symbol string `protobuf:"bytes,1,opt,name=symbol,proto3" json:"symbol,omitempty"`
	// The price of the symbol.
	Price string `protobuf:"bytes,2,opt,name=price,proto3" json:"price,omitempty"`
	// PriceOption defines the price option of a symbol.
	PriceOption PriceOption `protobuf:"varint,3,opt,name=price_option,json=priceOption,proto3,enum=query.PriceOption" json:"price_option,omitempty"`
}

func (x *PriceData) Reset() {
	*x = PriceData{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_query_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *PriceData) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PriceData) ProtoMessage() {}

func (x *PriceData) ProtoReflect() protoreflect.Message {
	mi := &file_proto_query_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use PriceData.ProtoReflect.Descriptor instead.
func (*PriceData) Descriptor() ([]byte, []int) {
	return file_proto_query_proto_rawDescGZIP(), []int{2}
}

func (x *PriceData) GetSymbol() string {
	if x != nil {
		return x.Symbol
	}
	return ""
}

func (x *PriceData) GetPrice() string {
	if x != nil {
		return x.Price
	}
	return ""
}

func (x *PriceData) GetPriceOption() PriceOption {
	if x != nil {
		return x.PriceOption
	}
	return PriceOption_PRICE_OPTION_UNSPECIFIED
}

var File_proto_query_proto protoreflect.FileDescriptor

var file_proto_query_proto_rawDesc = []byte{
	0x0a, 0x11, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x12, 0x05, 0x71, 0x75, 0x65, 0x72, 0x79, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e,
	0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x34,
	0x0a, 0x12, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x12, 0x1e, 0x0a, 0x07, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x73, 0x18,
	0x01, 0x20, 0x03, 0x28, 0x09, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x07, 0x73, 0x79, 0x6d,
	0x62, 0x6f, 0x6c, 0x73, 0x22, 0x45, 0x0a, 0x13, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x72, 0x69,
	0x63, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2e, 0x0a, 0x06, 0x70,
	0x72, 0x69, 0x63, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x71, 0x75,
	0x65, 0x72, 0x79, 0x2e, 0x50, 0x72, 0x69, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61, 0x42, 0x04, 0xc8,
	0xde, 0x1f, 0x00, 0x52, 0x06, 0x70, 0x72, 0x69, 0x63, 0x65, 0x73, 0x22, 0x70, 0x0a, 0x09, 0x50,
	0x72, 0x69, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x79, 0x6d, 0x62,
	0x6f, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c,
	0x12, 0x14, 0x0a, 0x05, 0x70, 0x72, 0x69, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x05, 0x70, 0x72, 0x69, 0x63, 0x65, 0x12, 0x35, 0x0a, 0x0c, 0x70, 0x72, 0x69, 0x63, 0x65, 0x5f,
	0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x71,
	0x75, 0x65, 0x72, 0x79, 0x2e, 0x50, 0x72, 0x69, 0x63, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
	0x52, 0x0b, 0x70, 0x72, 0x69, 0x63, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x2a, 0xf7, 0x01,
	0x0a, 0x0b, 0x50, 0x72, 0x69, 0x63, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x38, 0x0a,
	0x18, 0x50, 0x52, 0x49, 0x43, 0x45, 0x5f, 0x4f, 0x50, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e,
	0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x1a, 0x1a, 0x8a, 0x9d, 0x20,
	0x16, 0x50, 0x72, 0x69, 0x63, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x55, 0x6e, 0x73, 0x70,
	0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x12, 0x38, 0x0a, 0x18, 0x50, 0x52, 0x49, 0x43, 0x45,
	0x5f, 0x4f, 0x50, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e, 0x53, 0x55, 0x50, 0x50, 0x4f, 0x52,
	0x54, 0x45, 0x44, 0x10, 0x01, 0x1a, 0x1a, 0x8a, 0x9d, 0x20, 0x16, 0x50, 0x72, 0x69, 0x63, 0x65,
	0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65,
	0x64, 0x12, 0x38, 0x0a, 0x18, 0x50, 0x52, 0x49, 0x43, 0x45, 0x5f, 0x4f, 0x50, 0x54, 0x49, 0x4f,
	0x4e, 0x5f, 0x55, 0x4e, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x02, 0x1a,
	0x1a, 0x8a, 0x9d, 0x20, 0x16, 0x50, 0x72, 0x69, 0x63, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
	0x55, 0x6e, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x34, 0x0a, 0x16, 0x50,
	0x52, 0x49, 0x43, 0x45, 0x5f, 0x4f, 0x50, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x41, 0x56, 0x41, 0x49,
	0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x03, 0x1a, 0x18, 0x8a, 0x9d, 0x20, 0x14, 0x50, 0x72, 0x69,
	0x63, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c,
	0x65, 0x1a, 0x04, 0x88, 0xa3, 0x1e, 0x00, 0x32, 0x63, 0x0a, 0x05, 0x51, 0x75, 0x65, 0x72, 0x79,
	0x12, 0x5a, 0x0a, 0x06, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x12, 0x19, 0x2e, 0x71, 0x75, 0x65,
	0x72, 0x79, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x52, 0x65,
	0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1a, 0x2e, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x51, 0x75,
	0x65, 0x72, 0x79, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
	0x65, 0x22, 0x19, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x13, 0x12, 0x11, 0x2f, 0x70, 0x72, 0x69, 0x63,
	0x65, 0x73, 0x2f, 0x7b, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x73, 0x7d, 0x42, 0x12, 0x5a, 0x10,
	0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2d, 0x61, 0x70, 0x69, 0x2f, 0x71, 0x75, 0x65, 0x72, 0x79,
	0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_proto_query_proto_rawDescOnce sync.Once
	file_proto_query_proto_rawDescData = file_proto_query_proto_rawDesc
)

func file_proto_query_proto_rawDescGZIP() []byte {
	file_proto_query_proto_rawDescOnce.Do(func() {
		file_proto_query_proto_rawDescData = protoimpl.X.CompressGZIP(file_proto_query_proto_rawDescData)
	})
	return file_proto_query_proto_rawDescData
}

var file_proto_query_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_proto_query_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_proto_query_proto_goTypes = []interface{}{
	(PriceOption)(0),            // 0: query.PriceOption
	(*QueryPricesRequest)(nil),  // 1: query.QueryPricesRequest
	(*QueryPricesResponse)(nil), // 2: query.QueryPricesResponse
	(*PriceData)(nil),           // 3: query.PriceData
}
var file_proto_query_proto_depIdxs = []int32{
	3, // 0: query.QueryPricesResponse.prices:type_name -> query.PriceData
	0, // 1: query.PriceData.price_option:type_name -> query.PriceOption
	1, // 2: query.Query.Prices:input_type -> query.QueryPricesRequest
	2, // 3: query.Query.Prices:output_type -> query.QueryPricesResponse
	3, // [3:4] is the sub-list for method output_type
	2, // [2:3] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_proto_query_proto_init() }
func file_proto_query_proto_init() {
	if File_proto_query_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_proto_query_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
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
		file_proto_query_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
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
		file_proto_query_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*PriceData); i {
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
			RawDescriptor: file_proto_query_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_proto_query_proto_goTypes,
		DependencyIndexes: file_proto_query_proto_depIdxs,
		EnumInfos:         file_proto_query_proto_enumTypes,
		MessageInfos:      file_proto_query_proto_msgTypes,
	}.Build()
	File_proto_query_proto = out.File
	file_proto_query_proto_rawDesc = nil
	file_proto_query_proto_goTypes = nil
	file_proto_query_proto_depIdxs = nil
}
