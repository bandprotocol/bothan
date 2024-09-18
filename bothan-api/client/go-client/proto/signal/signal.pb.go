// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.0
// 	protoc        (unknown)
// source: proto/signal/signal.proto

package signal

import (
	_ "google.golang.org/genproto/googleapis/api/annotations"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	emptypb "google.golang.org/protobuf/types/known/emptypb"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// UpdateRegistryRequest is the request message for the UpdateRegistry RPC method.
// It contains the IPFS hash and version information needed to update the registry.
type UpdateRegistryRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The IPFS hash pointing to the registry data.
	IpfsHash string `protobuf:"bytes,1,opt,name=ipfs_hash,json=ipfsHash,proto3" json:"ipfs_hash,omitempty"`
	// The version of the registry.
	Version string `protobuf:"bytes,2,opt,name=version,proto3" json:"version,omitempty"`
}

func (x *UpdateRegistryRequest) Reset() {
	*x = UpdateRegistryRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_signal_signal_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UpdateRegistryRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UpdateRegistryRequest) ProtoMessage() {}

func (x *UpdateRegistryRequest) ProtoReflect() protoreflect.Message {
	mi := &file_proto_signal_signal_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UpdateRegistryRequest.ProtoReflect.Descriptor instead.
func (*UpdateRegistryRequest) Descriptor() ([]byte, []int) {
	return file_proto_signal_signal_proto_rawDescGZIP(), []int{0}
}

func (x *UpdateRegistryRequest) GetIpfsHash() string {
	if x != nil {
		return x.IpfsHash
	}
	return ""
}

func (x *UpdateRegistryRequest) GetVersion() string {
	if x != nil {
		return x.Version
	}
	return ""
}

// SetActiveSignalIdsRequest is the request message for the SetActiveSignalIds RPC method.
// It contains the list of signal IDs that should be marked as active.
type SetActiveSignalIdsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// A list of signal IDs to be set as active.
	SignalIds []string `protobuf:"bytes,1,rep,name=signal_ids,json=signalIds,proto3" json:"signal_ids,omitempty"`
}

func (x *SetActiveSignalIdsRequest) Reset() {
	*x = SetActiveSignalIdsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_signal_signal_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SetActiveSignalIdsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SetActiveSignalIdsRequest) ProtoMessage() {}

func (x *SetActiveSignalIdsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_proto_signal_signal_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SetActiveSignalIdsRequest.ProtoReflect.Descriptor instead.
func (*SetActiveSignalIdsRequest) Descriptor() ([]byte, []int) {
	return file_proto_signal_signal_proto_rawDescGZIP(), []int{1}
}

func (x *SetActiveSignalIdsRequest) GetSignalIds() []string {
	if x != nil {
		return x.SignalIds
	}
	return nil
}

var File_proto_signal_signal_proto protoreflect.FileDescriptor

var file_proto_signal_signal_proto_rawDesc = []byte{
	0x0a, 0x19, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x2f, 0x73,
	0x69, 0x67, 0x6e, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06, 0x73, 0x69, 0x67,
	0x6e, 0x61, 0x6c, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f,
	0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x1a, 0x1b, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
	0x75, 0x66, 0x2f, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4e,
	0x0a, 0x15, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x70, 0x66, 0x73, 0x5f,
	0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x69, 0x70, 0x66, 0x73,
	0x48, 0x61, 0x73, 0x68, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x3a,
	0x0a, 0x19, 0x53, 0x65, 0x74, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x53, 0x69, 0x67, 0x6e, 0x61,
	0x6c, 0x49, 0x64, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x73,
	0x69, 0x67, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52,
	0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x73, 0x32, 0xd1, 0x01, 0x0a, 0x0d, 0x53,
	0x69, 0x67, 0x6e, 0x61, 0x6c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x5a, 0x0a, 0x0e,
	0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x12, 0x1d,
	0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65,
	0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x16, 0x2e,
	0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e,
	0x45, 0x6d, 0x70, 0x74, 0x79, 0x22, 0x11, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x0b, 0x22, 0x09, 0x2f,
	0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x12, 0x64, 0x0a, 0x12, 0x53, 0x65, 0x74, 0x41,
	0x63, 0x74, 0x69, 0x76, 0x65, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x73, 0x12, 0x21,
	0x2e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x2e, 0x53, 0x65, 0x74, 0x41, 0x63, 0x74, 0x69, 0x76,
	0x65, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
	0x74, 0x1a, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x22, 0x13, 0x82, 0xd3, 0xe4, 0x93, 0x02,
	0x0d, 0x22, 0x0b, 0x2f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x73, 0x42, 0x32,
	0x5a, 0x30, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x62, 0x6f, 0x74,
	0x68, 0x61, 0x6e, 0x2f, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2d, 0x61, 0x70, 0x69, 0x2f, 0x63,
	0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x73, 0x69, 0x67, 0x6e,
	0x61, 0x6c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_proto_signal_signal_proto_rawDescOnce sync.Once
	file_proto_signal_signal_proto_rawDescData = file_proto_signal_signal_proto_rawDesc
)

func file_proto_signal_signal_proto_rawDescGZIP() []byte {
	file_proto_signal_signal_proto_rawDescOnce.Do(func() {
		file_proto_signal_signal_proto_rawDescData = protoimpl.X.CompressGZIP(file_proto_signal_signal_proto_rawDescData)
	})
	return file_proto_signal_signal_proto_rawDescData
}

var file_proto_signal_signal_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_proto_signal_signal_proto_goTypes = []interface{}{
	(*UpdateRegistryRequest)(nil),     // 0: signal.UpdateRegistryRequest
	(*SetActiveSignalIdsRequest)(nil), // 1: signal.SetActiveSignalIdsRequest
	(*emptypb.Empty)(nil),             // 2: google.protobuf.Empty
}
var file_proto_signal_signal_proto_depIdxs = []int32{
	0, // 0: signal.SignalService.UpdateRegistry:input_type -> signal.UpdateRegistryRequest
	1, // 1: signal.SignalService.SetActiveSignalIds:input_type -> signal.SetActiveSignalIdsRequest
	2, // 2: signal.SignalService.UpdateRegistry:output_type -> google.protobuf.Empty
	2, // 3: signal.SignalService.SetActiveSignalIds:output_type -> google.protobuf.Empty
	2, // [2:4] is the sub-list for method output_type
	0, // [0:2] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_proto_signal_signal_proto_init() }
func file_proto_signal_signal_proto_init() {
	if File_proto_signal_signal_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_proto_signal_signal_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UpdateRegistryRequest); i {
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
		file_proto_signal_signal_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SetActiveSignalIdsRequest); i {
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
			RawDescriptor: file_proto_signal_signal_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_proto_signal_signal_proto_goTypes,
		DependencyIndexes: file_proto_signal_signal_proto_depIdxs,
		MessageInfos:      file_proto_signal_signal_proto_msgTypes,
	}.Build()
	File_proto_signal_signal_proto = out.File
	file_proto_signal_signal_proto_rawDesc = nil
	file_proto_signal_signal_proto_goTypes = nil
	file_proto_signal_signal_proto_depIdxs = nil
}
