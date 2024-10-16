// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        (unknown)
// source: sf/substreams/sink/entity/v1/entity.proto

package pbentity

import (
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

type EntityChange_Operation int32

const (
	EntityChange_OPERATION_UNSPECIFIED EntityChange_Operation = 0 // Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
	EntityChange_OPERATION_CREATE      EntityChange_Operation = 1
	EntityChange_OPERATION_UPDATE      EntityChange_Operation = 2
	EntityChange_OPERATION_DELETE      EntityChange_Operation = 3
	EntityChange_OPERATION_FINAL       EntityChange_Operation = 4
)

// Enum value maps for EntityChange_Operation.
var (
	EntityChange_Operation_name = map[int32]string{
		0: "OPERATION_UNSPECIFIED",
		1: "OPERATION_CREATE",
		2: "OPERATION_UPDATE",
		3: "OPERATION_DELETE",
		4: "OPERATION_FINAL",
	}
	EntityChange_Operation_value = map[string]int32{
		"OPERATION_UNSPECIFIED": 0,
		"OPERATION_CREATE":      1,
		"OPERATION_UPDATE":      2,
		"OPERATION_DELETE":      3,
		"OPERATION_FINAL":       4,
	}
)

func (x EntityChange_Operation) Enum() *EntityChange_Operation {
	p := new(EntityChange_Operation)
	*p = x
	return p
}

func (x EntityChange_Operation) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (EntityChange_Operation) Descriptor() protoreflect.EnumDescriptor {
	return file_sf_substreams_sink_entity_v1_entity_proto_enumTypes[0].Descriptor()
}

func (EntityChange_Operation) Type() protoreflect.EnumType {
	return &file_sf_substreams_sink_entity_v1_entity_proto_enumTypes[0]
}

func (x EntityChange_Operation) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use EntityChange_Operation.Descriptor instead.
func (EntityChange_Operation) EnumDescriptor() ([]byte, []int) {
	return file_sf_substreams_sink_entity_v1_entity_proto_rawDescGZIP(), []int{1, 0}
}

type EntityChanges struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	EntityChanges []*EntityChange `protobuf:"bytes,5,rep,name=entity_changes,json=entityChanges,proto3" json:"entity_changes,omitempty"`
}

func (x *EntityChanges) Reset() {
	*x = EntityChanges{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *EntityChanges) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*EntityChanges) ProtoMessage() {}

func (x *EntityChanges) ProtoReflect() protoreflect.Message {
	mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use EntityChanges.ProtoReflect.Descriptor instead.
func (*EntityChanges) Descriptor() ([]byte, []int) {
	return file_sf_substreams_sink_entity_v1_entity_proto_rawDescGZIP(), []int{0}
}

func (x *EntityChanges) GetEntityChanges() []*EntityChange {
	if x != nil {
		return x.EntityChanges
	}
	return nil
}

type EntityChange struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Entity string `protobuf:"bytes,1,opt,name=entity,proto3" json:"entity,omitempty"`
	Id     string `protobuf:"bytes,2,opt,name=id,proto3" json:"id,omitempty"`
	// Deprecated, this is not used within `graph-node`.
	Ordinal   uint64                 `protobuf:"varint,3,opt,name=ordinal,proto3" json:"ordinal,omitempty"`
	Operation EntityChange_Operation `protobuf:"varint,4,opt,name=operation,proto3,enum=sf.substreams.sink.entity.v1.EntityChange_Operation" json:"operation,omitempty"`
	Fields    []*Field               `protobuf:"bytes,5,rep,name=fields,proto3" json:"fields,omitempty"`
}

func (x *EntityChange) Reset() {
	*x = EntityChange{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *EntityChange) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*EntityChange) ProtoMessage() {}

func (x *EntityChange) ProtoReflect() protoreflect.Message {
	mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use EntityChange.ProtoReflect.Descriptor instead.
func (*EntityChange) Descriptor() ([]byte, []int) {
	return file_sf_substreams_sink_entity_v1_entity_proto_rawDescGZIP(), []int{1}
}

func (x *EntityChange) GetEntity() string {
	if x != nil {
		return x.Entity
	}
	return ""
}

func (x *EntityChange) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *EntityChange) GetOrdinal() uint64 {
	if x != nil {
		return x.Ordinal
	}
	return 0
}

func (x *EntityChange) GetOperation() EntityChange_Operation {
	if x != nil {
		return x.Operation
	}
	return EntityChange_OPERATION_UNSPECIFIED
}

func (x *EntityChange) GetFields() []*Field {
	if x != nil {
		return x.Fields
	}
	return nil
}

type Value struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Typed:
	//
	//	*Value_Int32
	//	*Value_Bigdecimal
	//	*Value_Bigint
	//	*Value_String_
	//	*Value_Bytes
	//	*Value_Bool
	//	*Value_Timestamp
	//	*Value_Array
	Typed isValue_Typed `protobuf_oneof:"typed"`
}

func (x *Value) Reset() {
	*x = Value{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Value) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Value) ProtoMessage() {}

func (x *Value) ProtoReflect() protoreflect.Message {
	mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Value.ProtoReflect.Descriptor instead.
func (*Value) Descriptor() ([]byte, []int) {
	return file_sf_substreams_sink_entity_v1_entity_proto_rawDescGZIP(), []int{2}
}

func (m *Value) GetTyped() isValue_Typed {
	if m != nil {
		return m.Typed
	}
	return nil
}

func (x *Value) GetInt32() int32 {
	if x, ok := x.GetTyped().(*Value_Int32); ok {
		return x.Int32
	}
	return 0
}

func (x *Value) GetBigdecimal() string {
	if x, ok := x.GetTyped().(*Value_Bigdecimal); ok {
		return x.Bigdecimal
	}
	return ""
}

func (x *Value) GetBigint() string {
	if x, ok := x.GetTyped().(*Value_Bigint); ok {
		return x.Bigint
	}
	return ""
}

func (x *Value) GetString_() string {
	if x, ok := x.GetTyped().(*Value_String_); ok {
		return x.String_
	}
	return ""
}

func (x *Value) GetBytes() string {
	if x, ok := x.GetTyped().(*Value_Bytes); ok {
		return x.Bytes
	}
	return ""
}

func (x *Value) GetBool() bool {
	if x, ok := x.GetTyped().(*Value_Bool); ok {
		return x.Bool
	}
	return false
}

func (x *Value) GetTimestamp() int64 {
	if x, ok := x.GetTyped().(*Value_Timestamp); ok {
		return x.Timestamp
	}
	return 0
}

func (x *Value) GetArray() *Array {
	if x, ok := x.GetTyped().(*Value_Array); ok {
		return x.Array
	}
	return nil
}

type isValue_Typed interface {
	isValue_Typed()
}

type Value_Int32 struct {
	Int32 int32 `protobuf:"varint,1,opt,name=int32,proto3,oneof"`
}

type Value_Bigdecimal struct {
	Bigdecimal string `protobuf:"bytes,2,opt,name=bigdecimal,proto3,oneof"`
}

type Value_Bigint struct {
	Bigint string `protobuf:"bytes,3,opt,name=bigint,proto3,oneof"`
}

type Value_String_ struct {
	String_ string `protobuf:"bytes,4,opt,name=string,proto3,oneof"`
}

type Value_Bytes struct {
	Bytes string `protobuf:"bytes,5,opt,name=bytes,proto3,oneof"`
}

type Value_Bool struct {
	Bool bool `protobuf:"varint,6,opt,name=bool,proto3,oneof"`
}

type Value_Timestamp struct {
	Timestamp int64 `protobuf:"varint,7,opt,name=timestamp,proto3,oneof"`
}

type Value_Array struct {
	Array *Array `protobuf:"bytes,10,opt,name=array,proto3,oneof"`
}

func (*Value_Int32) isValue_Typed() {}

func (*Value_Bigdecimal) isValue_Typed() {}

func (*Value_Bigint) isValue_Typed() {}

func (*Value_String_) isValue_Typed() {}

func (*Value_Bytes) isValue_Typed() {}

func (*Value_Bool) isValue_Typed() {}

func (*Value_Timestamp) isValue_Typed() {}

func (*Value_Array) isValue_Typed() {}

type Array struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Value []*Value `protobuf:"bytes,1,rep,name=value,proto3" json:"value,omitempty"`
}

func (x *Array) Reset() {
	*x = Array{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Array) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Array) ProtoMessage() {}

func (x *Array) ProtoReflect() protoreflect.Message {
	mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Array.ProtoReflect.Descriptor instead.
func (*Array) Descriptor() ([]byte, []int) {
	return file_sf_substreams_sink_entity_v1_entity_proto_rawDescGZIP(), []int{3}
}

func (x *Array) GetValue() []*Value {
	if x != nil {
		return x.Value
	}
	return nil
}

type Field struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name     string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	NewValue *Value `protobuf:"bytes,3,opt,name=new_value,json=newValue,proto3,oneof" json:"new_value,omitempty"`
	// Deprecated, this is not used within `graph-node`.
	OldValue *Value `protobuf:"bytes,5,opt,name=old_value,json=oldValue,proto3,oneof" json:"old_value,omitempty"`
}

func (x *Field) Reset() {
	*x = Field{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Field) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Field) ProtoMessage() {}

func (x *Field) ProtoReflect() protoreflect.Message {
	mi := &file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Field.ProtoReflect.Descriptor instead.
func (*Field) Descriptor() ([]byte, []int) {
	return file_sf_substreams_sink_entity_v1_entity_proto_rawDescGZIP(), []int{4}
}

func (x *Field) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Field) GetNewValue() *Value {
	if x != nil {
		return x.NewValue
	}
	return nil
}

func (x *Field) GetOldValue() *Value {
	if x != nil {
		return x.OldValue
	}
	return nil
}

var File_sf_substreams_sink_entity_v1_entity_proto protoreflect.FileDescriptor

var file_sf_substreams_sink_entity_v1_entity_proto_rawDesc = []byte{
	0x0a, 0x29, 0x73, 0x66, 0x2f, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2f,
	0x73, 0x69, 0x6e, 0x6b, 0x2f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x65,
	0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1c, 0x73, 0x66, 0x2e,
	0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e,
	0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x22, 0x62, 0x0a, 0x0d, 0x45, 0x6e, 0x74,
	0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x12, 0x51, 0x0a, 0x0e, 0x65, 0x6e,
	0x74, 0x69, 0x74, 0x79, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x05, 0x20, 0x03,
	0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61,
	0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76,
	0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x0d,
	0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x22, 0xe0, 0x02,
	0x0a, 0x0c, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x16,
	0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
	0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61,
	0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c,
	0x12, 0x52, 0x0a, 0x09, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20,
	0x01, 0x28, 0x0e, 0x32, 0x34, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65,
	0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e,
	0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2e,
	0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x09, 0x6f, 0x70, 0x65, 0x72, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x12, 0x3b, 0x0a, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x05,
	0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72,
	0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
	0x2e, 0x76, 0x31, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x52, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64,
	0x73, 0x22, 0x7d, 0x0a, 0x09, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x19,
	0x0a, 0x15, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e, 0x53, 0x50,
	0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x14, 0x0a, 0x10, 0x4f, 0x50, 0x45,
	0x52, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x10, 0x01, 0x12,
	0x14, 0x0a, 0x10, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x50, 0x44,
	0x41, 0x54, 0x45, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x49,
	0x4f, 0x4e, 0x5f, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x10, 0x03, 0x12, 0x13, 0x0a, 0x0f, 0x4f,
	0x50, 0x45, 0x52, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x46, 0x49, 0x4e, 0x41, 0x4c, 0x10, 0x04,
	0x22, 0x89, 0x02, 0x0a, 0x05, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x16, 0x0a, 0x05, 0x69, 0x6e,
	0x74, 0x33, 0x32, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x48, 0x00, 0x52, 0x05, 0x69, 0x6e, 0x74,
	0x33, 0x32, 0x12, 0x20, 0x0a, 0x0a, 0x62, 0x69, 0x67, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0a, 0x62, 0x69, 0x67, 0x64, 0x65, 0x63,
	0x69, 0x6d, 0x61, 0x6c, 0x12, 0x18, 0x0a, 0x06, 0x62, 0x69, 0x67, 0x69, 0x6e, 0x74, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x06, 0x62, 0x69, 0x67, 0x69, 0x6e, 0x74, 0x12, 0x18,
	0x0a, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
	0x52, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x12, 0x16, 0x0a, 0x05, 0x62, 0x79, 0x74, 0x65,
	0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x05, 0x62, 0x79, 0x74, 0x65, 0x73,
	0x12, 0x14, 0x0a, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00,
	0x52, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x12, 0x1e, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
	0x61, 0x6d, 0x70, 0x18, 0x07, 0x20, 0x01, 0x28, 0x03, 0x48, 0x00, 0x52, 0x09, 0x74, 0x69, 0x6d,
	0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x3b, 0x0a, 0x05, 0x61, 0x72, 0x72, 0x61, 0x79, 0x18,
	0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74,
	0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74,
	0x79, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x72, 0x72, 0x61, 0x79, 0x48, 0x00, 0x52, 0x05, 0x61, 0x72,
	0x72, 0x61, 0x79, 0x42, 0x07, 0x0a, 0x05, 0x74, 0x79, 0x70, 0x65, 0x64, 0x22, 0x42, 0x0a, 0x05,
	0x41, 0x72, 0x72, 0x61, 0x79, 0x12, 0x39, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01,
	0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72,
	0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
	0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
	0x22, 0xc5, 0x01, 0x0a, 0x05, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
	0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x45,
	0x0a, 0x09, 0x6e, 0x65, 0x77, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x23, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
	0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31,
	0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x48, 0x00, 0x52, 0x08, 0x6e, 0x65, 0x77, 0x56, 0x61, 0x6c,
	0x75, 0x65, 0x88, 0x01, 0x01, 0x12, 0x45, 0x0a, 0x09, 0x6f, 0x6c, 0x64, 0x5f, 0x76, 0x61, 0x6c,
	0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75,
	0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x65, 0x6e,
	0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x48, 0x01, 0x52,
	0x08, 0x6f, 0x6c, 0x64, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x88, 0x01, 0x01, 0x42, 0x0c, 0x0a, 0x0a,
	0x5f, 0x6e, 0x65, 0x77, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x0c, 0x0a, 0x0a, 0x5f, 0x6f,
	0x6c, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x42, 0xa6, 0x02, 0x0a, 0x20, 0x63, 0x6f, 0x6d,
	0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73,
	0x69, 0x6e, 0x6b, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x42, 0x0b, 0x45,
	0x6e, 0x74, 0x69, 0x74, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x60, 0x67, 0x69,
	0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69,
	0x6e, 0x67, 0x66, 0x61, 0x73, 0x74, 0x2f, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
	0x73, 0x2d, 0x73, 0x69, 0x6e, 0x6b, 0x2d, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2d, 0x63, 0x68,
	0x61, 0x6e, 0x67, 0x65, 0x73, 0x2f, 0x70, 0x62, 0x2f, 0x73, 0x66, 0x2f, 0x73, 0x75, 0x62, 0x73,
	0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2f, 0x73, 0x69, 0x6e, 0x6b, 0x2f, 0x65, 0x6e, 0x74, 0x69,
	0x74, 0x79, 0x2f, 0x76, 0x31, 0x3b, 0x70, 0x62, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0xa2, 0x02,
	0x04, 0x53, 0x53, 0x53, 0x45, 0xaa, 0x02, 0x1c, 0x53, 0x66, 0x2e, 0x53, 0x75, 0x62, 0x73, 0x74,
	0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x53, 0x69, 0x6e, 0x6b, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74,
	0x79, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x1c, 0x53, 0x66, 0x5c, 0x53, 0x75, 0x62, 0x73, 0x74, 0x72,
	0x65, 0x61, 0x6d, 0x73, 0x5c, 0x53, 0x69, 0x6e, 0x6b, 0x5c, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
	0x5c, 0x56, 0x31, 0xe2, 0x02, 0x28, 0x53, 0x66, 0x5c, 0x53, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65,
	0x61, 0x6d, 0x73, 0x5c, 0x53, 0x69, 0x6e, 0x6b, 0x5c, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5c,
	0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02,
	0x20, 0x53, 0x66, 0x3a, 0x3a, 0x53, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x3a,
	0x3a, 0x53, 0x69, 0x6e, 0x6b, 0x3a, 0x3a, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x3a, 0x3a, 0x56,
	0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_sf_substreams_sink_entity_v1_entity_proto_rawDescOnce sync.Once
	file_sf_substreams_sink_entity_v1_entity_proto_rawDescData = file_sf_substreams_sink_entity_v1_entity_proto_rawDesc
)

func file_sf_substreams_sink_entity_v1_entity_proto_rawDescGZIP() []byte {
	file_sf_substreams_sink_entity_v1_entity_proto_rawDescOnce.Do(func() {
		file_sf_substreams_sink_entity_v1_entity_proto_rawDescData = protoimpl.X.CompressGZIP(file_sf_substreams_sink_entity_v1_entity_proto_rawDescData)
	})
	return file_sf_substreams_sink_entity_v1_entity_proto_rawDescData
}

var file_sf_substreams_sink_entity_v1_entity_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_sf_substreams_sink_entity_v1_entity_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_sf_substreams_sink_entity_v1_entity_proto_goTypes = []interface{}{
	(EntityChange_Operation)(0), // 0: sf.substreams.sink.entity.v1.EntityChange.Operation
	(*EntityChanges)(nil),       // 1: sf.substreams.sink.entity.v1.EntityChanges
	(*EntityChange)(nil),        // 2: sf.substreams.sink.entity.v1.EntityChange
	(*Value)(nil),               // 3: sf.substreams.sink.entity.v1.Value
	(*Array)(nil),               // 4: sf.substreams.sink.entity.v1.Array
	(*Field)(nil),               // 5: sf.substreams.sink.entity.v1.Field
}
var file_sf_substreams_sink_entity_v1_entity_proto_depIdxs = []int32{
	2, // 0: sf.substreams.sink.entity.v1.EntityChanges.entity_changes:type_name -> sf.substreams.sink.entity.v1.EntityChange
	0, // 1: sf.substreams.sink.entity.v1.EntityChange.operation:type_name -> sf.substreams.sink.entity.v1.EntityChange.Operation
	5, // 2: sf.substreams.sink.entity.v1.EntityChange.fields:type_name -> sf.substreams.sink.entity.v1.Field
	4, // 3: sf.substreams.sink.entity.v1.Value.array:type_name -> sf.substreams.sink.entity.v1.Array
	3, // 4: sf.substreams.sink.entity.v1.Array.value:type_name -> sf.substreams.sink.entity.v1.Value
	3, // 5: sf.substreams.sink.entity.v1.Field.new_value:type_name -> sf.substreams.sink.entity.v1.Value
	3, // 6: sf.substreams.sink.entity.v1.Field.old_value:type_name -> sf.substreams.sink.entity.v1.Value
	7, // [7:7] is the sub-list for method output_type
	7, // [7:7] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_sf_substreams_sink_entity_v1_entity_proto_init() }
func file_sf_substreams_sink_entity_v1_entity_proto_init() {
	if File_sf_substreams_sink_entity_v1_entity_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*EntityChanges); i {
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
		file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*EntityChange); i {
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
		file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Value); i {
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
		file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Array); i {
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
		file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Field); i {
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
	file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[2].OneofWrappers = []interface{}{
		(*Value_Int32)(nil),
		(*Value_Bigdecimal)(nil),
		(*Value_Bigint)(nil),
		(*Value_String_)(nil),
		(*Value_Bytes)(nil),
		(*Value_Bool)(nil),
		(*Value_Timestamp)(nil),
		(*Value_Array)(nil),
	}
	file_sf_substreams_sink_entity_v1_entity_proto_msgTypes[4].OneofWrappers = []interface{}{}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_sf_substreams_sink_entity_v1_entity_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_sf_substreams_sink_entity_v1_entity_proto_goTypes,
		DependencyIndexes: file_sf_substreams_sink_entity_v1_entity_proto_depIdxs,
		EnumInfos:         file_sf_substreams_sink_entity_v1_entity_proto_enumTypes,
		MessageInfos:      file_sf_substreams_sink_entity_v1_entity_proto_msgTypes,
	}.Build()
	File_sf_substreams_sink_entity_v1_entity_proto = out.File
	file_sf_substreams_sink_entity_v1_entity_proto_rawDesc = nil
	file_sf_substreams_sink_entity_v1_entity_proto_goTypes = nil
	file_sf_substreams_sink_entity_v1_entity_proto_depIdxs = nil
}
