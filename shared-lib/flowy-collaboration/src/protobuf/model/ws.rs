// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `ws.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct DocumentClientWSData {
    // message fields
    pub doc_id: ::std::string::String,
    pub ty: DocumentClientWSDataType,
    pub revisions: ::protobuf::SingularPtrField<super::revision::RepeatedRevision>,
    pub id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DocumentClientWSData {
    fn default() -> &'a DocumentClientWSData {
        <DocumentClientWSData as ::protobuf::Message>::default_instance()
    }
}

impl DocumentClientWSData {
    pub fn new() -> DocumentClientWSData {
        ::std::default::Default::default()
    }

    // string doc_id = 1;


    pub fn get_doc_id(&self) -> &str {
        &self.doc_id
    }
    pub fn clear_doc_id(&mut self) {
        self.doc_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_doc_id(&mut self, v: ::std::string::String) {
        self.doc_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doc_id(&mut self) -> &mut ::std::string::String {
        &mut self.doc_id
    }

    // Take field
    pub fn take_doc_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.doc_id, ::std::string::String::new())
    }

    // .DocumentClientWSDataType ty = 2;


    pub fn get_ty(&self) -> DocumentClientWSDataType {
        self.ty
    }
    pub fn clear_ty(&mut self) {
        self.ty = DocumentClientWSDataType::ClientPushRev;
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: DocumentClientWSDataType) {
        self.ty = v;
    }

    // .RepeatedRevision revisions = 3;


    pub fn get_revisions(&self) -> &super::revision::RepeatedRevision {
        self.revisions.as_ref().unwrap_or_else(|| <super::revision::RepeatedRevision as ::protobuf::Message>::default_instance())
    }
    pub fn clear_revisions(&mut self) {
        self.revisions.clear();
    }

    pub fn has_revisions(&self) -> bool {
        self.revisions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revisions(&mut self, v: super::revision::RepeatedRevision) {
        self.revisions = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revisions(&mut self) -> &mut super::revision::RepeatedRevision {
        if self.revisions.is_none() {
            self.revisions.set_default();
        }
        self.revisions.as_mut().unwrap()
    }

    // Take field
    pub fn take_revisions(&mut self) -> super::revision::RepeatedRevision {
        self.revisions.take().unwrap_or_else(|| super::revision::RepeatedRevision::new())
    }

    // string id = 4;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DocumentClientWSData {
    fn is_initialized(&self) -> bool {
        for v in &self.revisions {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.doc_id)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.ty, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.revisions)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.doc_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.doc_id);
        }
        if self.ty != DocumentClientWSDataType::ClientPushRev {
            my_size += ::protobuf::rt::enum_size(2, self.ty);
        }
        if let Some(ref v) = self.revisions.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.doc_id.is_empty() {
            os.write_string(1, &self.doc_id)?;
        }
        if self.ty != DocumentClientWSDataType::ClientPushRev {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.ty))?;
        }
        if let Some(ref v) = self.revisions.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.id.is_empty() {
            os.write_string(4, &self.id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DocumentClientWSData {
        DocumentClientWSData::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "doc_id",
                |m: &DocumentClientWSData| { &m.doc_id },
                |m: &mut DocumentClientWSData| { &mut m.doc_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DocumentClientWSDataType>>(
                "ty",
                |m: &DocumentClientWSData| { &m.ty },
                |m: &mut DocumentClientWSData| { &mut m.ty },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::revision::RepeatedRevision>>(
                "revisions",
                |m: &DocumentClientWSData| { &m.revisions },
                |m: &mut DocumentClientWSData| { &mut m.revisions },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &DocumentClientWSData| { &m.id },
                |m: &mut DocumentClientWSData| { &mut m.id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DocumentClientWSData>(
                "DocumentClientWSData",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DocumentClientWSData {
        static instance: ::protobuf::rt::LazyV2<DocumentClientWSData> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DocumentClientWSData::new)
    }
}

impl ::protobuf::Clear for DocumentClientWSData {
    fn clear(&mut self) {
        self.doc_id.clear();
        self.ty = DocumentClientWSDataType::ClientPushRev;
        self.revisions.clear();
        self.id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DocumentClientWSData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DocumentClientWSData {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DocumentServerWSData {
    // message fields
    pub doc_id: ::std::string::String,
    pub ty: DocumentServerWSDataType,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DocumentServerWSData {
    fn default() -> &'a DocumentServerWSData {
        <DocumentServerWSData as ::protobuf::Message>::default_instance()
    }
}

impl DocumentServerWSData {
    pub fn new() -> DocumentServerWSData {
        ::std::default::Default::default()
    }

    // string doc_id = 1;


    pub fn get_doc_id(&self) -> &str {
        &self.doc_id
    }
    pub fn clear_doc_id(&mut self) {
        self.doc_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_doc_id(&mut self, v: ::std::string::String) {
        self.doc_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doc_id(&mut self) -> &mut ::std::string::String {
        &mut self.doc_id
    }

    // Take field
    pub fn take_doc_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.doc_id, ::std::string::String::new())
    }

    // .DocumentServerWSDataType ty = 2;


    pub fn get_ty(&self) -> DocumentServerWSDataType {
        self.ty
    }
    pub fn clear_ty(&mut self) {
        self.ty = DocumentServerWSDataType::ServerAck;
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: DocumentServerWSDataType) {
        self.ty = v;
    }

    // bytes data = 3;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for DocumentServerWSData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.doc_id)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.ty, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.doc_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.doc_id);
        }
        if self.ty != DocumentServerWSDataType::ServerAck {
            my_size += ::protobuf::rt::enum_size(2, self.ty);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.doc_id.is_empty() {
            os.write_string(1, &self.doc_id)?;
        }
        if self.ty != DocumentServerWSDataType::ServerAck {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.ty))?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DocumentServerWSData {
        DocumentServerWSData::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "doc_id",
                |m: &DocumentServerWSData| { &m.doc_id },
                |m: &mut DocumentServerWSData| { &mut m.doc_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DocumentServerWSDataType>>(
                "ty",
                |m: &DocumentServerWSData| { &m.ty },
                |m: &mut DocumentServerWSData| { &mut m.ty },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "data",
                |m: &DocumentServerWSData| { &m.data },
                |m: &mut DocumentServerWSData| { &mut m.data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DocumentServerWSData>(
                "DocumentServerWSData",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DocumentServerWSData {
        static instance: ::protobuf::rt::LazyV2<DocumentServerWSData> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DocumentServerWSData::new)
    }
}

impl ::protobuf::Clear for DocumentServerWSData {
    fn clear(&mut self) {
        self.doc_id.clear();
        self.ty = DocumentServerWSDataType::ServerAck;
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DocumentServerWSData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DocumentServerWSData {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NewDocumentUser {
    // message fields
    pub user_id: ::std::string::String,
    pub doc_id: ::std::string::String,
    pub revision_data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NewDocumentUser {
    fn default() -> &'a NewDocumentUser {
        <NewDocumentUser as ::protobuf::Message>::default_instance()
    }
}

impl NewDocumentUser {
    pub fn new() -> NewDocumentUser {
        ::std::default::Default::default()
    }

    // string user_id = 1;


    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::string::String) {
        self.user_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_id(&mut self) -> &mut ::std::string::String {
        &mut self.user_id
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_id, ::std::string::String::new())
    }

    // string doc_id = 2;


    pub fn get_doc_id(&self) -> &str {
        &self.doc_id
    }
    pub fn clear_doc_id(&mut self) {
        self.doc_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_doc_id(&mut self, v: ::std::string::String) {
        self.doc_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doc_id(&mut self) -> &mut ::std::string::String {
        &mut self.doc_id
    }

    // Take field
    pub fn take_doc_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.doc_id, ::std::string::String::new())
    }

    // bytes revision_data = 3;


    pub fn get_revision_data(&self) -> &[u8] {
        &self.revision_data
    }
    pub fn clear_revision_data(&mut self) {
        self.revision_data.clear();
    }

    // Param is passed by value, moved
    pub fn set_revision_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.revision_data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.revision_data
    }

    // Take field
    pub fn take_revision_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.revision_data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for NewDocumentUser {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.doc_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.revision_data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_id);
        }
        if !self.doc_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.doc_id);
        }
        if !self.revision_data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.revision_data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.user_id.is_empty() {
            os.write_string(1, &self.user_id)?;
        }
        if !self.doc_id.is_empty() {
            os.write_string(2, &self.doc_id)?;
        }
        if !self.revision_data.is_empty() {
            os.write_bytes(3, &self.revision_data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> NewDocumentUser {
        NewDocumentUser::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "user_id",
                |m: &NewDocumentUser| { &m.user_id },
                |m: &mut NewDocumentUser| { &mut m.user_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "doc_id",
                |m: &NewDocumentUser| { &m.doc_id },
                |m: &mut NewDocumentUser| { &mut m.doc_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "revision_data",
                |m: &NewDocumentUser| { &m.revision_data },
                |m: &mut NewDocumentUser| { &mut m.revision_data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<NewDocumentUser>(
                "NewDocumentUser",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static NewDocumentUser {
        static instance: ::protobuf::rt::LazyV2<NewDocumentUser> = ::protobuf::rt::LazyV2::INIT;
        instance.get(NewDocumentUser::new)
    }
}

impl ::protobuf::Clear for NewDocumentUser {
    fn clear(&mut self) {
        self.user_id.clear();
        self.doc_id.clear();
        self.revision_data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewDocumentUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewDocumentUser {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DocumentClientWSDataType {
    ClientPushRev = 0,
    ClientPing = 1,
}

impl ::protobuf::ProtobufEnum for DocumentClientWSDataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DocumentClientWSDataType> {
        match value {
            0 => ::std::option::Option::Some(DocumentClientWSDataType::ClientPushRev),
            1 => ::std::option::Option::Some(DocumentClientWSDataType::ClientPing),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DocumentClientWSDataType] = &[
            DocumentClientWSDataType::ClientPushRev,
            DocumentClientWSDataType::ClientPing,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<DocumentClientWSDataType>("DocumentClientWSDataType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for DocumentClientWSDataType {
}

impl ::std::default::Default for DocumentClientWSDataType {
    fn default() -> Self {
        DocumentClientWSDataType::ClientPushRev
    }
}

impl ::protobuf::reflect::ProtobufValue for DocumentClientWSDataType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DocumentServerWSDataType {
    ServerAck = 0,
    ServerPushRev = 1,
    ServerPullRev = 2,
    UserConnect = 3,
}

impl ::protobuf::ProtobufEnum for DocumentServerWSDataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DocumentServerWSDataType> {
        match value {
            0 => ::std::option::Option::Some(DocumentServerWSDataType::ServerAck),
            1 => ::std::option::Option::Some(DocumentServerWSDataType::ServerPushRev),
            2 => ::std::option::Option::Some(DocumentServerWSDataType::ServerPullRev),
            3 => ::std::option::Option::Some(DocumentServerWSDataType::UserConnect),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DocumentServerWSDataType] = &[
            DocumentServerWSDataType::ServerAck,
            DocumentServerWSDataType::ServerPushRev,
            DocumentServerWSDataType::ServerPullRev,
            DocumentServerWSDataType::UserConnect,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<DocumentServerWSDataType>("DocumentServerWSDataType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for DocumentServerWSDataType {
}

impl ::std::default::Default for DocumentServerWSDataType {
    fn default() -> Self {
        DocumentServerWSDataType::ServerAck
    }
}

impl ::protobuf::reflect::ProtobufValue for DocumentServerWSDataType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08ws.proto\x1a\x0erevision.proto\"\x99\x01\n\x14DocumentClientWSData\
    \x12\x15\n\x06doc_id\x18\x01\x20\x01(\tR\x05docId\x12)\n\x02ty\x18\x02\
    \x20\x01(\x0e2\x19.DocumentClientWSDataTypeR\x02ty\x12/\n\trevisions\x18\
    \x03\x20\x01(\x0b2\x11.RepeatedRevisionR\trevisions\x12\x0e\n\x02id\x18\
    \x04\x20\x01(\tR\x02id\"l\n\x14DocumentServerWSData\x12\x15\n\x06doc_id\
    \x18\x01\x20\x01(\tR\x05docId\x12)\n\x02ty\x18\x02\x20\x01(\x0e2\x19.Doc\
    umentServerWSDataTypeR\x02ty\x12\x12\n\x04data\x18\x03\x20\x01(\x0cR\x04\
    data\"f\n\x0fNewDocumentUser\x12\x17\n\x07user_id\x18\x01\x20\x01(\tR\
    \x06userId\x12\x15\n\x06doc_id\x18\x02\x20\x01(\tR\x05docId\x12#\n\rrevi\
    sion_data\x18\x03\x20\x01(\x0cR\x0crevisionData*=\n\x18DocumentClientWSD\
    ataType\x12\x11\n\rClientPushRev\x10\0\x12\x0e\n\nClientPing\x10\x01*`\n\
    \x18DocumentServerWSDataType\x12\r\n\tServerAck\x10\0\x12\x11\n\rServerP\
    ushRev\x10\x01\x12\x11\n\rServerPullRev\x10\x02\x12\x0f\n\x0bUserConnect\
    \x10\x03J\xb1\x07\n\x06\x12\x04\0\0\x1c\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\t\n\x02\x03\0\x12\x03\x01\0\x18\n\n\n\x02\x04\0\x12\x04\x03\0\x08\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\x03\x08\x1c\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x04\x04\x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x04\x04\n\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x04\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x04\x14\x15\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x05\x04$\n\x0c\n\x05\
    \x04\0\x02\x01\x06\x12\x03\x05\x04\x1c\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x05\x1d\x1f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x05\"#\n\x0b\n\
    \x04\x04\0\x02\x02\x12\x03\x06\x04#\n\x0c\n\x05\x04\0\x02\x02\x06\x12\
    \x03\x06\x04\x14\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x06\x15\x1e\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x03\x06!\"\n\x0b\n\x04\x04\0\x02\x03\x12\
    \x03\x07\x04\x12\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x07\x04\n\n\x0c\n\
    \x05\x04\0\x02\x03\x01\x12\x03\x07\x0b\r\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x03\x07\x10\x11\n\n\n\x02\x04\x01\x12\x04\t\0\r\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\t\x08\x1c\n\x0b\n\x04\x04\x01\x02\0\x12\x03\n\x04\x16\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03\n\x04\n\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\n\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\n\x14\x15\n\
    \x0b\n\x04\x04\x01\x02\x01\x12\x03\x0b\x04$\n\x0c\n\x05\x04\x01\x02\x01\
    \x06\x12\x03\x0b\x04\x1c\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0b\x1d\
    \x1f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0b\"#\n\x0b\n\x04\x04\x01\
    \x02\x02\x12\x03\x0c\x04\x13\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x0c\
    \x04\t\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x0c\n\x0e\n\x0c\n\x05\x04\
    \x01\x02\x02\x03\x12\x03\x0c\x11\x12\n\n\n\x02\x04\x02\x12\x04\x0e\0\x12\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x0e\x08\x17\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03\x0f\x04\x17\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x0f\x04\n\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0f\x0b\x12\n\x0c\n\x05\x04\x02\x02\
    \0\x03\x12\x03\x0f\x15\x16\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x10\x04\
    \x16\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x10\x04\n\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03\x10\x0b\x11\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x10\x14\x15\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x11\x04\x1c\n\x0c\n\
    \x05\x04\x02\x02\x02\x05\x12\x03\x11\x04\t\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03\x11\n\x17\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x11\x1a\
    \x1b\n\n\n\x02\x05\0\x12\x04\x13\0\x16\x01\n\n\n\x03\x05\0\x01\x12\x03\
    \x13\x05\x1d\n\x0b\n\x04\x05\0\x02\0\x12\x03\x14\x04\x16\n\x0c\n\x05\x05\
    \0\x02\0\x01\x12\x03\x14\x04\x11\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x14\
    \x14\x15\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x15\x04\x13\n\x0c\n\x05\x05\0\
    \x02\x01\x01\x12\x03\x15\x04\x0e\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\
    \x15\x11\x12\n\n\n\x02\x05\x01\x12\x04\x17\0\x1c\x01\n\n\n\x03\x05\x01\
    \x01\x12\x03\x17\x05\x1d\n\x0b\n\x04\x05\x01\x02\0\x12\x03\x18\x04\x12\n\
    \x0c\n\x05\x05\x01\x02\0\x01\x12\x03\x18\x04\r\n\x0c\n\x05\x05\x01\x02\0\
    \x02\x12\x03\x18\x10\x11\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\x19\x04\x16\
    \n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03\x19\x04\x11\n\x0c\n\x05\x05\x01\
    \x02\x01\x02\x12\x03\x19\x14\x15\n\x0b\n\x04\x05\x01\x02\x02\x12\x03\x1a\
    \x04\x16\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03\x1a\x04\x11\n\x0c\n\x05\
    \x05\x01\x02\x02\x02\x12\x03\x1a\x14\x15\n\x0b\n\x04\x05\x01\x02\x03\x12\
    \x03\x1b\x04\x14\n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x03\x1b\x04\x0f\n\
    \x0c\n\x05\x05\x01\x02\x03\x02\x12\x03\x1b\x12\x13b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}