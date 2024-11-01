// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.0
// 	protoc        (unknown)
// source: bothan/v1/bothan.proto

package proto

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

// Status defines the status for a signal ID.
type Status int32

const (
	// Default status, should not be used.
	Status_STATUS_UNSPECIFIED Status = 0
	// Indicates that the signal ID is not supported.
	Status_STATUS_UNSUPPORTED Status = 1
	// Indicates that the signal ID is currently unavailable.
	Status_STATUS_UNAVAILABLE Status = 2
	// Indicates that the signal ID is available.
	Status_STATUS_AVAILABLE Status = 3
)

// Enum value maps for Status.
var (
	Status_name = map[int32]string{
		0: "STATUS_UNSPECIFIED",
		1: "STATUS_UNSUPPORTED",
		2: "STATUS_UNAVAILABLE",
		3: "STATUS_AVAILABLE",
	}
	Status_value = map[string]int32{
		"STATUS_UNSPECIFIED": 0,
		"STATUS_UNSUPPORTED": 1,
		"STATUS_UNAVAILABLE": 2,
		"STATUS_AVAILABLE":   3,
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
	return file_bothan_v1_bothan_proto_enumTypes[0].Descriptor()
}

func (Status) Type() protoreflect.EnumType {
	return &file_bothan_v1_bothan_proto_enumTypes[0]
}

func (x Status) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use Status.Descriptor instead.
func (Status) EnumDescriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{0}
}

// GetInfoRequest defines the request message for the GetInfo RPC method.
type GetInfoRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *GetInfoRequest) Reset() {
	*x = GetInfoRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_bothan_v1_bothan_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetInfoRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetInfoRequest) ProtoMessage() {}

func (x *GetInfoRequest) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetInfoRequest.ProtoReflect.Descriptor instead.
func (*GetInfoRequest) Descriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{0}
}

// GetInfoResponse defines the response message for the GetInfo RPC method.
type GetInfoResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The bothan version
	BothanVersion string `protobuf:"bytes,1,opt,name=bothan_version,json=bothanVersion,proto3" json:"bothan_version,omitempty"`
	// The IPFS hash pointing to the registry data.
	RegistryIpfsHash string `protobuf:"bytes,2,opt,name=registry_ipfs_hash,json=registryIpfsHash,proto3" json:"registry_ipfs_hash,omitempty"`
	// The version requirements for the registry.
	RegistryVersionRequirement string `protobuf:"bytes,3,opt,name=registry_version_requirement,json=registryVersionRequirement,proto3" json:"registry_version_requirement,omitempty"`
}

func (x *GetInfoResponse) Reset() {
	*x = GetInfoResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_bothan_v1_bothan_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetInfoResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetInfoResponse) ProtoMessage() {}

func (x *GetInfoResponse) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetInfoResponse.ProtoReflect.Descriptor instead.
func (*GetInfoResponse) Descriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{1}
}

func (x *GetInfoResponse) GetBothanVersion() string {
	if x != nil {
		return x.BothanVersion
	}
	return ""
}

func (x *GetInfoResponse) GetRegistryIpfsHash() string {
	if x != nil {
		return x.RegistryIpfsHash
	}
	return ""
}

func (x *GetInfoResponse) GetRegistryVersionRequirement() string {
	if x != nil {
		return x.RegistryVersionRequirement
	}
	return ""
}

// UpdateRegistryRequest defines the request message for the UpdateRegistry RPC method.
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
		mi := &file_bothan_v1_bothan_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UpdateRegistryRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UpdateRegistryRequest) ProtoMessage() {}

func (x *UpdateRegistryRequest) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[2]
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
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{2}
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

// UpdateRegistryResponse defines the response message for the UpdateRegistry RPC method.
type UpdateRegistryResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *UpdateRegistryResponse) Reset() {
	*x = UpdateRegistryResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_bothan_v1_bothan_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UpdateRegistryResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UpdateRegistryResponse) ProtoMessage() {}

func (x *UpdateRegistryResponse) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UpdateRegistryResponse.ProtoReflect.Descriptor instead.
func (*UpdateRegistryResponse) Descriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{3}
}

// PushMonitoringRecordsRequest defines the request message for the PushMonitoringRecords RPC method.
type PushMonitoringRecordsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The uuid of a list of monitoring records to be pushed to the monitoring service.
	Uuid string `protobuf:"bytes,1,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// The tx hash of the transaction associated with the monitoring records.
	TxHash string `protobuf:"bytes,2,opt,name=tx_hash,json=txHash,proto3" json:"tx_hash,omitempty"`
}

func (x *PushMonitoringRecordsRequest) Reset() {
	*x = PushMonitoringRecordsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_bothan_v1_bothan_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *PushMonitoringRecordsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PushMonitoringRecordsRequest) ProtoMessage() {}

func (x *PushMonitoringRecordsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use PushMonitoringRecordsRequest.ProtoReflect.Descriptor instead.
func (*PushMonitoringRecordsRequest) Descriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{4}
}

func (x *PushMonitoringRecordsRequest) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *PushMonitoringRecordsRequest) GetTxHash() string {
	if x != nil {
		return x.TxHash
	}
	return ""
}

// PushMonitoringRecordsResponse defines the response message for the PushMonitoringRecords RPC method.
type PushMonitoringRecordsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *PushMonitoringRecordsResponse) Reset() {
	*x = PushMonitoringRecordsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_bothan_v1_bothan_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *PushMonitoringRecordsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PushMonitoringRecordsResponse) ProtoMessage() {}

func (x *PushMonitoringRecordsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use PushMonitoringRecordsResponse.ProtoReflect.Descriptor instead.
func (*PushMonitoringRecordsResponse) Descriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{5}
}

// GetPricesRequest defines the request message for the GetPrices RPC method.
type GetPricesRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// A list of signal IDs for which the prices are being requested.
	SignalIds []string `protobuf:"bytes,1,rep,name=signal_ids,json=signalIds,proto3" json:"signal_ids,omitempty"`
}

func (x *GetPricesRequest) Reset() {
	*x = GetPricesRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_bothan_v1_bothan_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetPricesRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetPricesRequest) ProtoMessage() {}

func (x *GetPricesRequest) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetPricesRequest.ProtoReflect.Descriptor instead.
func (*GetPricesRequest) Descriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{6}
}

func (x *GetPricesRequest) GetSignalIds() []string {
	if x != nil {
		return x.SignalIds
	}
	return nil
}

// GetPricesResponse defines the response message for the GetPrices RPC method.
type GetPricesResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// A unique identifier for the response.
	Uuid string `protobuf:"bytes,1,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// A list of prices for the requested signal IDs.
	Prices []*Price `protobuf:"bytes,2,rep,name=prices,proto3" json:"prices,omitempty"`
}

func (x *GetPricesResponse) Reset() {
	*x = GetPricesResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_bothan_v1_bothan_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetPricesResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetPricesResponse) ProtoMessage() {}

func (x *GetPricesResponse) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetPricesResponse.ProtoReflect.Descriptor instead.
func (*GetPricesResponse) Descriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{7}
}

func (x *GetPricesResponse) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *GetPricesResponse) GetPrices() []*Price {
	if x != nil {
		return x.Prices
	}
	return nil
}

// Price defines the price information for a signal ID.
type Price struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The signal ID.
	SignalId string `protobuf:"bytes,1,opt,name=signal_id,json=signalId,proto3" json:"signal_id,omitempty"`
	// The price value associated with this signal ID.
	Price uint64 `protobuf:"varint,2,opt,name=price,proto3" json:"price,omitempty"`
	// The status of the signal ID.
	Status Status `protobuf:"varint,3,opt,name=status,proto3,enum=bothan.v1.Status" json:"status,omitempty"`
}

func (x *Price) Reset() {
	*x = Price{}
	if protoimpl.UnsafeEnabled {
		mi := &file_bothan_v1_bothan_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Price) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Price) ProtoMessage() {}

func (x *Price) ProtoReflect() protoreflect.Message {
	mi := &file_bothan_v1_bothan_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Price.ProtoReflect.Descriptor instead.
func (*Price) Descriptor() ([]byte, []int) {
	return file_bothan_v1_bothan_proto_rawDescGZIP(), []int{8}
}

func (x *Price) GetSignalId() string {
	if x != nil {
		return x.SignalId
	}
	return ""
}

func (x *Price) GetPrice() uint64 {
	if x != nil {
		return x.Price
	}
	return 0
}

func (x *Price) GetStatus() Status {
	if x != nil {
		return x.Status
	}
	return Status_STATUS_UNSPECIFIED
}

var File_bothan_v1_bothan_proto protoreflect.FileDescriptor

var file_bothan_v1_bothan_proto_rawDesc = []byte{
	0x0a, 0x16, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x62, 0x6f, 0x74, 0x68,
	0x61, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x09, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e,
	0x2e, 0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f,
	0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x22, 0x10, 0x0a, 0x0e, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71, 0x75,
	0x65, 0x73, 0x74, 0x22, 0xa8, 0x01, 0x0a, 0x0f, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52,
	0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x62, 0x6f, 0x74, 0x68, 0x61,
	0x6e, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x0d, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x2c,
	0x0a, 0x12, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x5f, 0x69, 0x70, 0x66, 0x73, 0x5f,
	0x68, 0x61, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x72, 0x65, 0x67, 0x69,
	0x73, 0x74, 0x72, 0x79, 0x49, 0x70, 0x66, 0x73, 0x48, 0x61, 0x73, 0x68, 0x12, 0x40, 0x0a, 0x1c,
	0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
	0x5f, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x1a, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x56, 0x65, 0x72, 0x73,
	0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x22, 0x4e,
	0x0a, 0x15, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x70, 0x66, 0x73, 0x5f,
	0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x69, 0x70, 0x66, 0x73,
	0x48, 0x61, 0x73, 0x68, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x18,
	0x0a, 0x16, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79,
	0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x4b, 0x0a, 0x1c, 0x50, 0x75, 0x73, 0x68,
	0x4d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64,
	0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x17, 0x0a, 0x07,
	0x74, 0x78, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x74,
	0x78, 0x48, 0x61, 0x73, 0x68, 0x22, 0x1f, 0x0a, 0x1d, 0x50, 0x75, 0x73, 0x68, 0x4d, 0x6f, 0x6e,
	0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x73, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x31, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x50, 0x72, 0x69,
	0x63, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x69,
	0x67, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x09,
	0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x73, 0x22, 0x51, 0x0a, 0x11, 0x47, 0x65, 0x74,
	0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12,
	0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75, 0x75,
	0x69, 0x64, 0x12, 0x28, 0x0a, 0x06, 0x70, 0x72, 0x69, 0x63, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03,
	0x28, 0x0b, 0x32, 0x10, 0x2e, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x50,
	0x72, 0x69, 0x63, 0x65, 0x52, 0x06, 0x70, 0x72, 0x69, 0x63, 0x65, 0x73, 0x22, 0x65, 0x0a, 0x05,
	0x50, 0x72, 0x69, 0x63, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x5f,
	0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c,
	0x49, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x72, 0x69, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x04, 0x52, 0x05, 0x70, 0x72, 0x69, 0x63, 0x65, 0x12, 0x29, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74,
	0x75, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x62, 0x6f, 0x74, 0x68, 0x61,
	0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61,
	0x74, 0x75, 0x73, 0x2a, 0x66, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x16, 0x0a,
	0x12, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46,
	0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x16, 0x0a, 0x12, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f,
	0x55, 0x4e, 0x53, 0x55, 0x50, 0x50, 0x4f, 0x52, 0x54, 0x45, 0x44, 0x10, 0x01, 0x12, 0x16, 0x0a,
	0x12, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x55, 0x4e, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41,
	0x42, 0x4c, 0x45, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f,
	0x41, 0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x03, 0x32, 0xba, 0x03, 0x0a, 0x0d,
	0x42, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x4f, 0x0a,
	0x07, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x19, 0x2e, 0x62, 0x6f, 0x74, 0x68, 0x61,
	0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71, 0x75,
	0x65, 0x73, 0x74, 0x1a, 0x1a, 0x2e, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2e, 0x76, 0x31, 0x2e,
	0x47, 0x65, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
	0x0d, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x07, 0x12, 0x05, 0x2f, 0x69, 0x6e, 0x66, 0x6f, 0x12, 0x68,
	0x0a, 0x0e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79,
	0x12, 0x20, 0x2e, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x70, 0x64,
	0x61, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65,
	0x73, 0x74, 0x1a, 0x21, 0x2e, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x55,
	0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x52, 0x65, 0x73,
	0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x11, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x0b, 0x22, 0x09, 0x2f,
	0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x12, 0x87, 0x01, 0x0a, 0x15, 0x50, 0x75, 0x73,
	0x68, 0x4d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x63, 0x6f, 0x72,
	0x64, 0x73, 0x12, 0x27, 0x2e, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x50,
	0x75, 0x73, 0x68, 0x4d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x63,
	0x6f, 0x72, 0x64, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x28, 0x2e, 0x62, 0x6f,
	0x74, 0x68, 0x61, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x75, 0x73, 0x68, 0x4d, 0x6f, 0x6e, 0x69,
	0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x73, 0x52, 0x65, 0x73,
	0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x1b, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x15, 0x22, 0x13, 0x2f,
	0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x72, 0x65, 0x63, 0x6f, 0x72,
	0x64, 0x73, 0x12, 0x64, 0x0a, 0x09, 0x47, 0x65, 0x74, 0x50, 0x72, 0x69, 0x63, 0x65, 0x73, 0x12,
	0x1b, 0x2e, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50,
	0x72, 0x69, 0x63, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1c, 0x2e, 0x62,
	0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x72, 0x69, 0x63,
	0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x1c, 0x82, 0xd3, 0xe4, 0x93,
	0x02, 0x16, 0x12, 0x14, 0x2f, 0x70, 0x72, 0x69, 0x63, 0x65, 0x73, 0x2f, 0x7b, 0x73, 0x69, 0x67,
	0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x73, 0x7d, 0x42, 0x2b, 0x5a, 0x29, 0x67, 0x69, 0x74, 0x68,
	0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x62, 0x6f, 0x74, 0x68, 0x61, 0x6e, 0x2f, 0x62, 0x6f,
	0x74, 0x68, 0x61, 0x6e, 0x2d, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2f,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_bothan_v1_bothan_proto_rawDescOnce sync.Once
	file_bothan_v1_bothan_proto_rawDescData = file_bothan_v1_bothan_proto_rawDesc
)

func file_bothan_v1_bothan_proto_rawDescGZIP() []byte {
	file_bothan_v1_bothan_proto_rawDescOnce.Do(func() {
		file_bothan_v1_bothan_proto_rawDescData = protoimpl.X.CompressGZIP(file_bothan_v1_bothan_proto_rawDescData)
	})
	return file_bothan_v1_bothan_proto_rawDescData
}

var file_bothan_v1_bothan_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_bothan_v1_bothan_proto_msgTypes = make([]protoimpl.MessageInfo, 9)
var file_bothan_v1_bothan_proto_goTypes = []interface{}{
	(Status)(0),                           // 0: bothan.v1.Status
	(*GetInfoRequest)(nil),                // 1: bothan.v1.GetInfoRequest
	(*GetInfoResponse)(nil),               // 2: bothan.v1.GetInfoResponse
	(*UpdateRegistryRequest)(nil),         // 3: bothan.v1.UpdateRegistryRequest
	(*UpdateRegistryResponse)(nil),        // 4: bothan.v1.UpdateRegistryResponse
	(*PushMonitoringRecordsRequest)(nil),  // 5: bothan.v1.PushMonitoringRecordsRequest
	(*PushMonitoringRecordsResponse)(nil), // 6: bothan.v1.PushMonitoringRecordsResponse
	(*GetPricesRequest)(nil),              // 7: bothan.v1.GetPricesRequest
	(*GetPricesResponse)(nil),             // 8: bothan.v1.GetPricesResponse
	(*Price)(nil),                         // 9: bothan.v1.Price
}
var file_bothan_v1_bothan_proto_depIdxs = []int32{
	9, // 0: bothan.v1.GetPricesResponse.prices:type_name -> bothan.v1.Price
	0, // 1: bothan.v1.Price.status:type_name -> bothan.v1.Status
	1, // 2: bothan.v1.BothanService.GetInfo:input_type -> bothan.v1.GetInfoRequest
	3, // 3: bothan.v1.BothanService.UpdateRegistry:input_type -> bothan.v1.UpdateRegistryRequest
	5, // 4: bothan.v1.BothanService.PushMonitoringRecords:input_type -> bothan.v1.PushMonitoringRecordsRequest
	7, // 5: bothan.v1.BothanService.GetPrices:input_type -> bothan.v1.GetPricesRequest
	2, // 6: bothan.v1.BothanService.GetInfo:output_type -> bothan.v1.GetInfoResponse
	4, // 7: bothan.v1.BothanService.UpdateRegistry:output_type -> bothan.v1.UpdateRegistryResponse
	6, // 8: bothan.v1.BothanService.PushMonitoringRecords:output_type -> bothan.v1.PushMonitoringRecordsResponse
	8, // 9: bothan.v1.BothanService.GetPrices:output_type -> bothan.v1.GetPricesResponse
	6, // [6:10] is the sub-list for method output_type
	2, // [2:6] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_bothan_v1_bothan_proto_init() }
func file_bothan_v1_bothan_proto_init() {
	if File_bothan_v1_bothan_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_bothan_v1_bothan_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetInfoRequest); i {
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
		file_bothan_v1_bothan_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetInfoResponse); i {
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
		file_bothan_v1_bothan_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
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
		file_bothan_v1_bothan_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UpdateRegistryResponse); i {
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
		file_bothan_v1_bothan_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*PushMonitoringRecordsRequest); i {
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
		file_bothan_v1_bothan_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*PushMonitoringRecordsResponse); i {
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
		file_bothan_v1_bothan_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetPricesRequest); i {
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
		file_bothan_v1_bothan_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetPricesResponse); i {
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
		file_bothan_v1_bothan_proto_msgTypes[8].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Price); i {
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
			RawDescriptor: file_bothan_v1_bothan_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   9,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_bothan_v1_bothan_proto_goTypes,
		DependencyIndexes: file_bothan_v1_bothan_proto_depIdxs,
		EnumInfos:         file_bothan_v1_bothan_proto_enumTypes,
		MessageInfos:      file_bothan_v1_bothan_proto_msgTypes,
	}.Build()
	File_bothan_v1_bothan_proto = out.File
	file_bothan_v1_bothan_proto_rawDesc = nil
	file_bothan_v1_bothan_proto_goTypes = nil
	file_bothan_v1_bothan_proto_depIdxs = nil
}
