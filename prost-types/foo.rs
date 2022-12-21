#![feature(prelude_import)]
#![doc(html_root_url = "https://docs.rs/prost-types/0.11.2")]
//! Protocol Buffers well-known types.
//!
//! Note that the documentation for the types defined in this crate are generated from the Protobuf
//! definitions, so code examples are not in Rust.
//!
//! See the [Protobuf reference][1] for more information about well-known types.
//!
//! [1]: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use core::convert::TryFrom;
use core::fmt;
use core::i32;
use core::i64;
use core::str::FromStr;
use core::time;
/// The protocol compiler can output a FileDescriptorSet containing the .proto
/// files it parses.
pub struct FileDescriptorSet {
    #[prost(message, repeated, tag = "1")]
    pub file: ::prost::alloc::vec::Vec<FileDescriptorProto>,
}
#[automatically_derived]
impl ::core::clone::Clone for FileDescriptorSet {
    #[inline]
    fn clone(&self) -> FileDescriptorSet {
        FileDescriptorSet {
            file: ::core::clone::Clone::clone(&self.file),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FileDescriptorSet {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FileDescriptorSet {
    #[inline]
    fn eq(&self, other: &FileDescriptorSet) -> bool {
        self.file == other.file
    }
}
impl ::prost::Message for FileDescriptorSet {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        for msg in &self.file {
            ::prost::encoding::message::encode(1u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "FileDescriptorSet";
        match tag {
            1u32 => {
                let mut value = &mut self.file;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "file");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.file)
    }
    fn clear(&mut self) {
        self.file.clear();
    }
}
impl ::core::default::Default for FileDescriptorSet {
    fn default() -> Self {
        FileDescriptorSet {
            file: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for FileDescriptorSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("FileDescriptorSet");
        let builder = {
            let wrapper = &self.file;
            builder.field("file", &wrapper)
        };
        builder.finish()
    }
}
/// Describes a complete .proto file.
pub struct FileDescriptorProto {
    /// file name, relative to root of source tree
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// e.g. "foo", "foo.bar", etc.
    #[prost(string, optional, tag = "2")]
    pub package: ::core::option::Option<::prost::alloc::string::String>,
    /// Names of files imported by this file.
    #[prost(string, repeated, tag = "3")]
    pub dependency: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indexes of the public imported files in the dependency list above.
    #[prost(int32, repeated, packed = "false", tag = "10")]
    pub public_dependency: ::prost::alloc::vec::Vec<i32>,
    /// Indexes of the weak imported files in the dependency list.
    /// For Google-internal migration only. Do not use.
    #[prost(int32, repeated, packed = "false", tag = "11")]
    pub weak_dependency: ::prost::alloc::vec::Vec<i32>,
    /// All top-level definitions in this file.
    #[prost(message, repeated, tag = "4")]
    pub message_type: ::prost::alloc::vec::Vec<DescriptorProto>,
    #[prost(message, repeated, tag = "5")]
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProto>,
    #[prost(message, repeated, tag = "6")]
    pub service: ::prost::alloc::vec::Vec<ServiceDescriptorProto>,
    #[prost(message, repeated, tag = "7")]
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, optional, tag = "8")]
    pub options: ::core::option::Option<FileOptions>,
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    #[prost(message, optional, tag = "9")]
    pub source_code_info: ::core::option::Option<SourceCodeInfo>,
    /// The syntax of the proto file.
    /// The supported values are "proto2" and "proto3".
    #[prost(string, optional, tag = "12")]
    pub syntax: ::core::option::Option<::prost::alloc::string::String>,
}
#[automatically_derived]
impl ::core::clone::Clone for FileDescriptorProto {
    #[inline]
    fn clone(&self) -> FileDescriptorProto {
        FileDescriptorProto {
            name: ::core::clone::Clone::clone(&self.name),
            package: ::core::clone::Clone::clone(&self.package),
            dependency: ::core::clone::Clone::clone(&self.dependency),
            public_dependency: ::core::clone::Clone::clone(&self.public_dependency),
            weak_dependency: ::core::clone::Clone::clone(&self.weak_dependency),
            message_type: ::core::clone::Clone::clone(&self.message_type),
            enum_type: ::core::clone::Clone::clone(&self.enum_type),
            service: ::core::clone::Clone::clone(&self.service),
            extension: ::core::clone::Clone::clone(&self.extension),
            options: ::core::clone::Clone::clone(&self.options),
            source_code_info: ::core::clone::Clone::clone(&self.source_code_info),
            syntax: ::core::clone::Clone::clone(&self.syntax),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FileDescriptorProto {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FileDescriptorProto {
    #[inline]
    fn eq(&self, other: &FileDescriptorProto) -> bool {
        self.name == other.name
            && self.package == other.package
            && self.dependency == other.dependency
            && self.public_dependency == other.public_dependency
            && self.weak_dependency == other.weak_dependency
            && self.message_type == other.message_type
            && self.enum_type == other.enum_type
            && self.service == other.service
            && self.extension == other.extension
            && self.options == other.options
            && self.source_code_info == other.source_code_info
            && self.syntax == other.syntax
    }
}
impl ::prost::Message for FileDescriptorProto {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.name, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(2u32, &self.package, buf);
        <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(3u32, &self.dependency, buf);
        for msg in &self.message_type {
            ::prost::encoding::message::encode(4u32, msg, buf);
        }
        for msg in &self.enum_type {
            ::prost::encoding::message::encode(5u32, msg, buf);
        }
        for msg in &self.service {
            ::prost::encoding::message::encode(6u32, msg, buf);
        }
        for msg in &self.extension {
            ::prost::encoding::message::encode(7u32, msg, buf);
        }
        if let Some(ref msg) = self.options {
            ::prost::encoding::message::encode(8u32, msg, buf);
        }
        if let Some(ref msg) = self.source_code_info {
            ::prost::encoding::message::encode(9u32, msg, buf);
        }
        <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::Int32,
            _,
            _,
        >(10u32, &self.public_dependency, buf);
        <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::Int32,
            _,
            _,
        >(11u32, &self.weak_dependency, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(12u32, &self.syntax, buf);
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "FileDescriptorProto";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.package;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "package");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.dependency;
                <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "dependency");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.message_type;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "message_type");
                        error
                    },
                )
            }
            5u32 => {
                let mut value = &mut self.enum_type;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "enum_type");
                        error
                    },
                )
            }
            6u32 => {
                let mut value = &mut self.service;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "service");
                        error
                    },
                )
            }
            7u32 => {
                let mut value = &mut self.extension;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "extension");
                        error
                    },
                )
            }
            8u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "options");
                    error
                })
            }
            9u32 => {
                let mut value = &mut self.source_code_info;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "source_code_info");
                    error
                })
            }
            10u32 => {
                let mut value = &mut self.public_dependency;
                <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "public_dependency");
                    error
                })
            }
            11u32 => {
                let mut value = &mut self.weak_dependency;
                <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "weak_dependency");
                    error
                })
            }
            12u32 => {
                let mut value = &mut self.syntax;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "syntax");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + self.package.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(2u32, value)
        }) + ::prost::encoding::string::encoded_len_repeated(3u32, &self.dependency)
            + ::prost::encoding::message::encoded_len_repeated(4u32, &self.message_type)
            + ::prost::encoding::message::encoded_len_repeated(5u32, &self.enum_type)
            + ::prost::encoding::message::encoded_len_repeated(6u32, &self.service)
            + ::prost::encoding::message::encoded_len_repeated(7u32, &self.extension)
            + self
                .options
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(8u32, msg))
            + self
                .source_code_info
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(9u32, msg))
            + ::prost::encoding::int32::encoded_len_repeated(10u32, &self.public_dependency)
            + ::prost::encoding::int32::encoded_len_repeated(11u32, &self.weak_dependency)
            + self.syntax.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(12u32, value)
            })
    }
    fn clear(&mut self) {
        self.name = ::core::option::Option::None;
        self.package = ::core::option::Option::None;
        self.dependency.clear();
        self.message_type.clear();
        self.enum_type.clear();
        self.service.clear();
        self.extension.clear();
        self.options = ::core::option::Option::None;
        self.source_code_info = ::core::option::Option::None;
        self.public_dependency.clear();
        self.weak_dependency.clear();
        self.syntax = ::core::option::Option::None;
    }
}
impl ::core::default::Default for FileDescriptorProto {
    fn default() -> Self {
        FileDescriptorProto {
            name: ::core::option::Option::None,
            package: ::core::option::Option::None,
            dependency: ::prost::alloc::vec::Vec::new(),
            message_type: ::core::default::Default::default(),
            enum_type: ::core::default::Default::default(),
            service: ::core::default::Default::default(),
            extension: ::core::default::Default::default(),
            options: ::core::default::Default::default(),
            source_code_info: ::core::default::Default::default(),
            public_dependency: ::prost::alloc::vec::Vec::new(),
            weak_dependency: ::prost::alloc::vec::Vec::new(),
            syntax: ::core::option::Option::None,
        }
    }
}
impl ::core::fmt::Debug for FileDescriptorProto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("FileDescriptorProto");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.package)
            };
            builder.field("package", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let mut vec_builder = f.debug_list();
                        for v in self.0 {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            vec_builder.entry(&Inner(v));
                        }
                        vec_builder.finish()
                    }
                }
                ScalarWrapper(&self.dependency)
            };
            builder.field("dependency", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let mut vec_builder = f.debug_list();
                        for v in self.0 {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            vec_builder.entry(&Inner(v));
                        }
                        vec_builder.finish()
                    }
                }
                ScalarWrapper(&self.public_dependency)
            };
            builder.field("public_dependency", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let mut vec_builder = f.debug_list();
                        for v in self.0 {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            vec_builder.entry(&Inner(v));
                        }
                        vec_builder.finish()
                    }
                }
                ScalarWrapper(&self.weak_dependency)
            };
            builder.field("weak_dependency", &wrapper)
        };
        let builder = {
            let wrapper = &self.message_type;
            builder.field("message_type", &wrapper)
        };
        let builder = {
            let wrapper = &self.enum_type;
            builder.field("enum_type", &wrapper)
        };
        let builder = {
            let wrapper = &self.service;
            builder.field("service", &wrapper)
        };
        let builder = {
            let wrapper = &self.extension;
            builder.field("extension", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = &self.source_code_info;
            builder.field("source_code_info", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.syntax)
            };
            builder.field("syntax", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl FileDescriptorProto {
    ///Returns the value of `name`, or the default value if `name` is unset.
    pub fn name(&self) -> &str {
        match self.name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `package`, or the default value if `package` is unset.
    pub fn package(&self) -> &str {
        match self.package {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `syntax`, or the default value if `syntax` is unset.
    pub fn syntax(&self) -> &str {
        match self.syntax {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
}
/// Describes a message type.
pub struct DescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub field: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, repeated, tag = "6")]
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, repeated, tag = "3")]
    pub nested_type: ::prost::alloc::vec::Vec<DescriptorProto>,
    #[prost(message, repeated, tag = "4")]
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProto>,
    #[prost(message, repeated, tag = "5")]
    pub extension_range: ::prost::alloc::vec::Vec<descriptor_proto::ExtensionRange>,
    #[prost(message, repeated, tag = "8")]
    pub oneof_decl: ::prost::alloc::vec::Vec<OneofDescriptorProto>,
    #[prost(message, optional, tag = "7")]
    pub options: ::core::option::Option<MessageOptions>,
    #[prost(message, repeated, tag = "9")]
    pub reserved_range: ::prost::alloc::vec::Vec<descriptor_proto::ReservedRange>,
    /// Reserved field names, which may not be used by fields in the same message.
    /// A given name may only be reserved once.
    #[prost(string, repeated, tag = "10")]
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[automatically_derived]
impl ::core::clone::Clone for DescriptorProto {
    #[inline]
    fn clone(&self) -> DescriptorProto {
        DescriptorProto {
            name: ::core::clone::Clone::clone(&self.name),
            field: ::core::clone::Clone::clone(&self.field),
            extension: ::core::clone::Clone::clone(&self.extension),
            nested_type: ::core::clone::Clone::clone(&self.nested_type),
            enum_type: ::core::clone::Clone::clone(&self.enum_type),
            extension_range: ::core::clone::Clone::clone(&self.extension_range),
            oneof_decl: ::core::clone::Clone::clone(&self.oneof_decl),
            options: ::core::clone::Clone::clone(&self.options),
            reserved_range: ::core::clone::Clone::clone(&self.reserved_range),
            reserved_name: ::core::clone::Clone::clone(&self.reserved_name),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for DescriptorProto {}
#[automatically_derived]
impl ::core::cmp::PartialEq for DescriptorProto {
    #[inline]
    fn eq(&self, other: &DescriptorProto) -> bool {
        self.name == other.name
            && self.field == other.field
            && self.extension == other.extension
            && self.nested_type == other.nested_type
            && self.enum_type == other.enum_type
            && self.extension_range == other.extension_range
            && self.oneof_decl == other.oneof_decl
            && self.options == other.options
            && self.reserved_range == other.reserved_range
            && self.reserved_name == other.reserved_name
    }
}
impl ::prost::Message for DescriptorProto {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.name, buf);
        for msg in &self.field {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
        for msg in &self.nested_type {
            ::prost::encoding::message::encode(3u32, msg, buf);
        }
        for msg in &self.enum_type {
            ::prost::encoding::message::encode(4u32, msg, buf);
        }
        for msg in &self.extension_range {
            ::prost::encoding::message::encode(5u32, msg, buf);
        }
        for msg in &self.extension {
            ::prost::encoding::message::encode(6u32, msg, buf);
        }
        if let Some(ref msg) = self.options {
            ::prost::encoding::message::encode(7u32, msg, buf);
        }
        for msg in &self.oneof_decl {
            ::prost::encoding::message::encode(8u32, msg, buf);
        }
        for msg in &self.reserved_range {
            ::prost::encoding::message::encode(9u32, msg, buf);
        }
        <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(10u32, &self.reserved_name, buf);
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "DescriptorProto";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.field;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "field");
                        error
                    },
                )
            }
            3u32 => {
                let mut value = &mut self.nested_type;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "nested_type");
                        error
                    },
                )
            }
            4u32 => {
                let mut value = &mut self.enum_type;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "enum_type");
                        error
                    },
                )
            }
            5u32 => {
                let mut value = &mut self.extension_range;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "extension_range");
                        error
                    },
                )
            }
            6u32 => {
                let mut value = &mut self.extension;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "extension");
                        error
                    },
                )
            }
            7u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "options");
                    error
                })
            }
            8u32 => {
                let mut value = &mut self.oneof_decl;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "oneof_decl");
                        error
                    },
                )
            }
            9u32 => {
                let mut value = &mut self.reserved_range;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "reserved_range");
                        error
                    },
                )
            }
            10u32 => {
                let mut value = &mut self.reserved_name;
                <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "reserved_name");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + ::prost::encoding::message::encoded_len_repeated(2u32, &self.field)
            + ::prost::encoding::message::encoded_len_repeated(3u32, &self.nested_type)
            + ::prost::encoding::message::encoded_len_repeated(4u32, &self.enum_type)
            + ::prost::encoding::message::encoded_len_repeated(5u32, &self.extension_range)
            + ::prost::encoding::message::encoded_len_repeated(6u32, &self.extension)
            + self
                .options
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(7u32, msg))
            + ::prost::encoding::message::encoded_len_repeated(8u32, &self.oneof_decl)
            + ::prost::encoding::message::encoded_len_repeated(9u32, &self.reserved_range)
            + ::prost::encoding::string::encoded_len_repeated(10u32, &self.reserved_name)
    }
    fn clear(&mut self) {
        self.name = ::core::option::Option::None;
        self.field.clear();
        self.nested_type.clear();
        self.enum_type.clear();
        self.extension_range.clear();
        self.extension.clear();
        self.options = ::core::option::Option::None;
        self.oneof_decl.clear();
        self.reserved_range.clear();
        self.reserved_name.clear();
    }
}
impl ::core::default::Default for DescriptorProto {
    fn default() -> Self {
        DescriptorProto {
            name: ::core::option::Option::None,
            field: ::core::default::Default::default(),
            nested_type: ::core::default::Default::default(),
            enum_type: ::core::default::Default::default(),
            extension_range: ::core::default::Default::default(),
            extension: ::core::default::Default::default(),
            options: ::core::default::Default::default(),
            oneof_decl: ::core::default::Default::default(),
            reserved_range: ::core::default::Default::default(),
            reserved_name: ::prost::alloc::vec::Vec::new(),
        }
    }
}
impl ::core::fmt::Debug for DescriptorProto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("DescriptorProto");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = &self.field;
            builder.field("field", &wrapper)
        };
        let builder = {
            let wrapper = &self.extension;
            builder.field("extension", &wrapper)
        };
        let builder = {
            let wrapper = &self.nested_type;
            builder.field("nested_type", &wrapper)
        };
        let builder = {
            let wrapper = &self.enum_type;
            builder.field("enum_type", &wrapper)
        };
        let builder = {
            let wrapper = &self.extension_range;
            builder.field("extension_range", &wrapper)
        };
        let builder = {
            let wrapper = &self.oneof_decl;
            builder.field("oneof_decl", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = &self.reserved_range;
            builder.field("reserved_range", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let mut vec_builder = f.debug_list();
                        for v in self.0 {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            vec_builder.entry(&Inner(v));
                        }
                        vec_builder.finish()
                    }
                }
                ScalarWrapper(&self.reserved_name)
            };
            builder.field("reserved_name", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl DescriptorProto {
    ///Returns the value of `name`, or the default value if `name` is unset.
    pub fn name(&self) -> &str {
        match self.name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
}
/// Nested message and enum types in `DescriptorProto`.
pub mod descriptor_proto {
    pub struct ExtensionRange {
        /// Inclusive.
        #[prost(int32, optional, tag = "1")]
        pub start: ::core::option::Option<i32>,
        /// Exclusive.
        #[prost(int32, optional, tag = "2")]
        pub end: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "3")]
        pub options: ::core::option::Option<super::ExtensionRangeOptions>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ExtensionRange {
        #[inline]
        fn clone(&self) -> ExtensionRange {
            ExtensionRange {
                start: ::core::clone::Clone::clone(&self.start),
                end: ::core::clone::Clone::clone(&self.end),
                options: ::core::clone::Clone::clone(&self.options),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ExtensionRange {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ExtensionRange {
        #[inline]
        fn eq(&self, other: &ExtensionRange) -> bool {
            self.start == other.start && self.end == other.end && self.options == other.options
        }
    }
    impl ::prost::Message for ExtensionRange {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(1u32, &self.start, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(2u32, &self.end, buf);
            if let Some(ref msg) = self.options {
                ::prost::encoding::message::encode(3u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "ExtensionRange";
            match tag {
                1u32 => {
                    let mut value = &mut self.start;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "start");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.end;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "end");
                        error
                    })
                }
                3u32 => {
                    let mut value = &mut self.options;
                    ::prost::encoding::message::merge(
                        wire_type,
                        value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "options");
                        error
                    })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.start.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(1u32, value)
            }) + self.end.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(2u32, value)
            }) + self
                .options
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(3u32, msg))
        }
        fn clear(&mut self) {
            self.start = ::core::option::Option::None;
            self.end = ::core::option::Option::None;
            self.options = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for ExtensionRange {
        fn default() -> Self {
            ExtensionRange {
                start: ::core::option::Option::None,
                end: ::core::option::Option::None,
                options: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for ExtensionRange {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ExtensionRange");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.start)
                };
                builder.field("start", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.end)
                };
                builder.field("end", &wrapper)
            };
            let builder = {
                let wrapper = &self.options;
                builder.field("options", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl ExtensionRange {
        ///Returns the value of `start`, or the default value if `start` is unset.
        pub fn start(&self) -> i32 {
            match self.start {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `end`, or the default value if `end` is unset.
        pub fn end(&self) -> i32 {
            match self.end {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
    }
    /// Range of reserved tag numbers. Reserved tag numbers may not be used by
    /// fields or extension ranges in the same message. Reserved ranges may
    /// not overlap.
    pub struct ReservedRange {
        /// Inclusive.
        #[prost(int32, optional, tag = "1")]
        pub start: ::core::option::Option<i32>,
        /// Exclusive.
        #[prost(int32, optional, tag = "2")]
        pub end: ::core::option::Option<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ReservedRange {
        #[inline]
        fn clone(&self) -> ReservedRange {
            ReservedRange {
                start: ::core::clone::Clone::clone(&self.start),
                end: ::core::clone::Clone::clone(&self.end),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ReservedRange {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ReservedRange {
        #[inline]
        fn eq(&self, other: &ReservedRange) -> bool {
            self.start == other.start && self.end == other.end
        }
    }
    impl ::prost::Message for ReservedRange {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(1u32, &self.start, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(2u32, &self.end, buf);
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "ReservedRange";
            match tag {
                1u32 => {
                    let mut value = &mut self.start;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "start");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.end;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "end");
                        error
                    })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.start.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(1u32, value)
            }) + self.end.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(2u32, value)
            })
        }
        fn clear(&mut self) {
            self.start = ::core::option::Option::None;
            self.end = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for ReservedRange {
        fn default() -> Self {
            ReservedRange {
                start: ::core::option::Option::None,
                end: ::core::option::Option::None,
            }
        }
    }
    impl ::core::fmt::Debug for ReservedRange {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ReservedRange");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.start)
                };
                builder.field("start", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.end)
                };
                builder.field("end", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl ReservedRange {
        ///Returns the value of `start`, or the default value if `start` is unset.
        pub fn start(&self) -> i32 {
            match self.start {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `end`, or the default value if `end` is unset.
        pub fn end(&self) -> i32 {
            match self.end {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
    }
}
pub struct ExtensionRangeOptions {
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for ExtensionRangeOptions {
    #[inline]
    fn clone(&self) -> ExtensionRangeOptions {
        ExtensionRangeOptions {
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ExtensionRangeOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ExtensionRangeOptions {
    #[inline]
    fn eq(&self, other: &ExtensionRangeOptions) -> bool {
        self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for ExtensionRangeOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "ExtensionRangeOptions";
        match tag {
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        ExtensionRangeOptions {
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for ExtensionRangeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("ExtensionRangeOptions");
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
/// Describes a field within a message.
pub struct FieldDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub number: ::core::option::Option<i32>,
    #[prost(enumeration = "field_descriptor_proto::Label", optional, tag = "4")]
    pub label: ::core::option::Option<i32>,
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    #[prost(enumeration = "field_descriptor_proto::Type", optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    #[prost(string, optional, tag = "6")]
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as type_name.
    #[prost(string, optional, tag = "2")]
    pub extendee: ::core::option::Option<::prost::alloc::string::String>,
    /// For numeric types, contains the original text representation of the value.
    /// For booleans, "true" or "false".
    /// For strings, contains the default text contents (not escaped in any way).
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    /// TODO(kenton):  Base-64 encode?
    #[prost(string, optional, tag = "7")]
    pub default_value: ::core::option::Option<::prost::alloc::string::String>,
    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.  This field is a member of that oneof.
    #[prost(int32, optional, tag = "9")]
    pub oneof_index: ::core::option::Option<i32>,
    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    #[prost(string, optional, tag = "10")]
    pub json_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub options: ::core::option::Option<FieldOptions>,
    /// If true, this is a proto3 "optional". When a proto3 field is optional, it
    /// tracks presence regardless of field type.
    ///
    /// When proto3_optional is true, this field must be belong to a oneof to
    /// signal to old proto3 clients that presence is tracked for this field. This
    /// oneof is known as a "synthetic" oneof, and this field must be its sole
    /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
    /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
    /// oneofs must be ordered after all "real" oneofs.
    ///
    /// For message fields, proto3_optional doesn't create any semantic change,
    /// since non-repeated message fields always track presence. However it still
    /// indicates the semantic detail of whether the user wrote "optional" or not.
    /// This can be useful for round-tripping the .proto file. For consistency we
    /// give message fields a synthetic oneof also, even though it is not required
    /// to track presence. This is especially important because the parser can't
    /// tell if a field is a message or an enum, so it must always create a
    /// synthetic oneof.
    ///
    /// Proto2 optional fields do not set this flag, because they already indicate
    /// optional with `LABEL_OPTIONAL`.
    #[prost(bool, optional, tag = "17")]
    pub proto3_optional: ::core::option::Option<bool>,
}
#[automatically_derived]
impl ::core::clone::Clone for FieldDescriptorProto {
    #[inline]
    fn clone(&self) -> FieldDescriptorProto {
        FieldDescriptorProto {
            name: ::core::clone::Clone::clone(&self.name),
            number: ::core::clone::Clone::clone(&self.number),
            label: ::core::clone::Clone::clone(&self.label),
            r#type: ::core::clone::Clone::clone(&self.r#type),
            type_name: ::core::clone::Clone::clone(&self.type_name),
            extendee: ::core::clone::Clone::clone(&self.extendee),
            default_value: ::core::clone::Clone::clone(&self.default_value),
            oneof_index: ::core::clone::Clone::clone(&self.oneof_index),
            json_name: ::core::clone::Clone::clone(&self.json_name),
            options: ::core::clone::Clone::clone(&self.options),
            proto3_optional: ::core::clone::Clone::clone(&self.proto3_optional),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FieldDescriptorProto {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FieldDescriptorProto {
    #[inline]
    fn eq(&self, other: &FieldDescriptorProto) -> bool {
        self.name == other.name
            && self.number == other.number
            && self.label == other.label
            && self.r#type == other.r#type
            && self.type_name == other.type_name
            && self.extendee == other.extendee
            && self.default_value == other.default_value
            && self.oneof_index == other.oneof_index
            && self.json_name == other.json_name
            && self.options == other.options
            && self.proto3_optional == other.proto3_optional
    }
}
impl ::prost::Message for FieldDescriptorProto {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.name, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(2u32, &self.extendee, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::Int32,
            _,
            _,
        >(3u32, &self.number, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufEnum,
            _,
            _,
        >(4u32, &self.label, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufEnum,
            _,
            _,
        >(5u32, &self.r#type, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(6u32, &self.type_name, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(7u32, &self.default_value, buf);
        if let Some(ref msg) = self.options {
            ::prost::encoding::message::encode(8u32, msg, buf);
        }
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::Int32,
            _,
            _,
        >(9u32, &self.oneof_index, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(10u32, &self.json_name, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(17u32, &self.proto3_optional, buf);
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "FieldDescriptorProto";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.extendee;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "extendee");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.number;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "number");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.label;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "label");
                    error
                })
            }
            5u32 => {
                let mut value = &mut self.r#type;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "r#type");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.type_name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "type_name");
                    error
                })
            }
            7u32 => {
                let mut value = &mut self.default_value;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "default_value");
                    error
                })
            }
            8u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "options");
                    error
                })
            }
            9u32 => {
                let mut value = &mut self.oneof_index;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "oneof_index");
                    error
                })
            }
            10u32 => {
                let mut value = &mut self.json_name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "json_name");
                    error
                })
            }
            17u32 => {
                let mut value = &mut self.proto3_optional;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "proto3_optional");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + self.extendee.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(2u32, value)
        }) + self.number.as_ref().map_or(0, |value| {
            ::prost::encoding::int32::encoded_len(3u32, value)
        }) + self.label.as_ref().map_or(0, |value| {
            ::prost::encoding::int32::encoded_len(4u32, value)
        }) + self.r#type.as_ref().map_or(0, |value| {
            ::prost::encoding::int32::encoded_len(5u32, value)
        }) + self.type_name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(6u32, value)
        }) + self.default_value.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(7u32, value)
        }) + self
            .options
            .as_ref()
            .map_or(0, |msg| ::prost::encoding::message::encoded_len(8u32, msg))
            + self.oneof_index.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(9u32, value)
            })
            + self.json_name.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(10u32, value)
            })
            + self.proto3_optional.as_ref().map_or(0, |value| {
                ::prost::encoding::bool::encoded_len(17u32, value)
            })
    }
    fn clear(&mut self) {
        self.name = ::core::option::Option::None;
        self.extendee = ::core::option::Option::None;
        self.number = ::core::option::Option::None;
        self.label = ::core::option::Option::None;
        self.r#type = ::core::option::Option::None;
        self.type_name = ::core::option::Option::None;
        self.default_value = ::core::option::Option::None;
        self.options = ::core::option::Option::None;
        self.oneof_index = ::core::option::Option::None;
        self.json_name = ::core::option::Option::None;
        self.proto3_optional = ::core::option::Option::None;
    }
}
impl ::core::default::Default for FieldDescriptorProto {
    fn default() -> Self {
        FieldDescriptorProto {
            name: ::core::option::Option::None,
            extendee: ::core::option::Option::None,
            number: ::core::option::Option::None,
            label: ::core::option::Option::None,
            r#type: ::core::option::Option::None,
            type_name: ::core::option::Option::None,
            default_value: ::core::option::Option::None,
            options: ::core::default::Default::default(),
            oneof_index: ::core::option::Option::None,
            json_name: ::core::option::Option::None,
            proto3_optional: ::core::option::Option::None,
        }
    }
}
impl ::core::fmt::Debug for FieldDescriptorProto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("FieldDescriptorProto");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.number)
            };
            builder.field("number", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        struct Inner<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for Inner<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                match field_descriptor_proto::Label::from_i32(*self.0) {
                                    None => ::core::fmt::Debug::fmt(&self.0, f),
                                    Some(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.label)
            };
            builder.field("label", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        struct Inner<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for Inner<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                match field_descriptor_proto::Type::from_i32(*self.0) {
                                    None => ::core::fmt::Debug::fmt(&self.0, f),
                                    Some(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.r#type)
            };
            builder.field("r#type", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.type_name)
            };
            builder.field("type_name", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.extendee)
            };
            builder.field("extendee", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.default_value)
            };
            builder.field("default_value", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.oneof_index)
            };
            builder.field("oneof_index", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.json_name)
            };
            builder.field("json_name", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.proto3_optional)
            };
            builder.field("proto3_optional", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl FieldDescriptorProto {
    ///Returns the value of `name`, or the default value if `name` is unset.
    pub fn name(&self) -> &str {
        match self.name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `extendee`, or the default value if `extendee` is unset.
    pub fn extendee(&self) -> &str {
        match self.extendee {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `number`, or the default value if `number` is unset.
    pub fn number(&self) -> i32 {
        match self.number {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => 0i32,
        }
    }
    ///Returns the enum value of `label`, or the default if the field is unset or set to an invalid enum value.
    pub fn label(&self) -> field_descriptor_proto::Label {
        self.label
            .and_then(field_descriptor_proto::Label::from_i32)
            .unwrap_or(field_descriptor_proto::Label::default())
    }
    ///Sets `label` to the provided enum value.
    pub fn set_label(&mut self, value: field_descriptor_proto::Label) {
        self.label = ::core::option::Option::Some(value as i32);
    }
    ///Returns the enum value of `type`, or the default if the field is unset or set to an invalid enum value.
    pub fn r#type(&self) -> field_descriptor_proto::Type {
        self.r#type
            .and_then(field_descriptor_proto::Type::from_i32)
            .unwrap_or(field_descriptor_proto::Type::default())
    }
    ///Sets `type` to the provided enum value.
    pub fn set_type(&mut self, value: field_descriptor_proto::Type) {
        self.r#type = ::core::option::Option::Some(value as i32);
    }
    ///Returns the value of `type_name`, or the default value if `type_name` is unset.
    pub fn type_name(&self) -> &str {
        match self.type_name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `default_value`, or the default value if `default_value` is unset.
    pub fn default_value(&self) -> &str {
        match self.default_value {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `oneof_index`, or the default value if `oneof_index` is unset.
    pub fn oneof_index(&self) -> i32 {
        match self.oneof_index {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => 0i32,
        }
    }
    ///Returns the value of `json_name`, or the default value if `json_name` is unset.
    pub fn json_name(&self) -> &str {
        match self.json_name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `proto3_optional`, or the default value if `proto3_optional` is unset.
    pub fn proto3_optional(&self) -> bool {
        match self.proto3_optional {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
}
/// Nested message and enum types in `FieldDescriptorProto`.
pub mod field_descriptor_proto {
    #[repr(i32)]
    pub enum Type {
        /// 0 is reserved for errors.
        /// Order is weird for historical reasons.
        Double = 1,
        Float = 2,
        /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
        /// negative values are likely.
        Int64 = 3,
        Uint64 = 4,
        /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
        /// negative values are likely.
        Int32 = 5,
        Fixed64 = 6,
        Fixed32 = 7,
        Bool = 8,
        String = 9,
        /// Tag-delimited aggregate.
        /// Group type is deprecated and not supported in proto3. However, Proto3
        /// implementations should still be able to parse the group wire format and
        /// treat group fields as unknown fields.
        Group = 10,
        /// Length-delimited aggregate.
        Message = 11,
        /// New in version 2.
        Bytes = 12,
        Uint32 = 13,
        Enum = 14,
        Sfixed32 = 15,
        Sfixed64 = 16,
        /// Uses ZigZag encoding.
        Sint32 = 17,
        /// Uses ZigZag encoding.
        Sint64 = 18,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Type {
        #[inline]
        fn clone(&self) -> Type {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Type {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Type {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Type::Double => ::core::fmt::Formatter::write_str(f, "Double"),
                Type::Float => ::core::fmt::Formatter::write_str(f, "Float"),
                Type::Int64 => ::core::fmt::Formatter::write_str(f, "Int64"),
                Type::Uint64 => ::core::fmt::Formatter::write_str(f, "Uint64"),
                Type::Int32 => ::core::fmt::Formatter::write_str(f, "Int32"),
                Type::Fixed64 => ::core::fmt::Formatter::write_str(f, "Fixed64"),
                Type::Fixed32 => ::core::fmt::Formatter::write_str(f, "Fixed32"),
                Type::Bool => ::core::fmt::Formatter::write_str(f, "Bool"),
                Type::String => ::core::fmt::Formatter::write_str(f, "String"),
                Type::Group => ::core::fmt::Formatter::write_str(f, "Group"),
                Type::Message => ::core::fmt::Formatter::write_str(f, "Message"),
                Type::Bytes => ::core::fmt::Formatter::write_str(f, "Bytes"),
                Type::Uint32 => ::core::fmt::Formatter::write_str(f, "Uint32"),
                Type::Enum => ::core::fmt::Formatter::write_str(f, "Enum"),
                Type::Sfixed32 => ::core::fmt::Formatter::write_str(f, "Sfixed32"),
                Type::Sfixed64 => ::core::fmt::Formatter::write_str(f, "Sfixed64"),
                Type::Sint32 => ::core::fmt::Formatter::write_str(f, "Sint32"),
                Type::Sint64 => ::core::fmt::Formatter::write_str(f, "Sint64"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Type {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Type {
        #[inline]
        fn eq(&self, other: &Type) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Type {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Type {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Type {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Type {
        #[inline]
        fn partial_cmp(&self, other: &Type) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Type {
        #[inline]
        fn cmp(&self, other: &Type) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    impl Type {
        ///Returns `true` if `value` is a variant of `Type`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                1 => true,
                2 => true,
                3 => true,
                4 => true,
                5 => true,
                6 => true,
                7 => true,
                8 => true,
                9 => true,
                10 => true,
                11 => true,
                12 => true,
                13 => true,
                14 => true,
                15 => true,
                16 => true,
                17 => true,
                18 => true,
                _ => false,
            }
        }
        ///Converts an `i32` to a `Type`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<Type> {
            match value {
                1 => ::core::option::Option::Some(Type::Double),
                2 => ::core::option::Option::Some(Type::Float),
                3 => ::core::option::Option::Some(Type::Int64),
                4 => ::core::option::Option::Some(Type::Uint64),
                5 => ::core::option::Option::Some(Type::Int32),
                6 => ::core::option::Option::Some(Type::Fixed64),
                7 => ::core::option::Option::Some(Type::Fixed32),
                8 => ::core::option::Option::Some(Type::Bool),
                9 => ::core::option::Option::Some(Type::String),
                10 => ::core::option::Option::Some(Type::Group),
                11 => ::core::option::Option::Some(Type::Message),
                12 => ::core::option::Option::Some(Type::Bytes),
                13 => ::core::option::Option::Some(Type::Uint32),
                14 => ::core::option::Option::Some(Type::Enum),
                15 => ::core::option::Option::Some(Type::Sfixed32),
                16 => ::core::option::Option::Some(Type::Sfixed64),
                17 => ::core::option::Option::Some(Type::Sint32),
                18 => ::core::option::Option::Some(Type::Sint64),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for Type {
        fn default() -> Type {
            Type::Double
        }
    }
    impl ::core::convert::From<Type> for i32 {
        fn from(value: Type) -> i32 {
            value as i32
        }
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Double => "TYPE_DOUBLE",
                Type::Float => "TYPE_FLOAT",
                Type::Int64 => "TYPE_INT64",
                Type::Uint64 => "TYPE_UINT64",
                Type::Int32 => "TYPE_INT32",
                Type::Fixed64 => "TYPE_FIXED64",
                Type::Fixed32 => "TYPE_FIXED32",
                Type::Bool => "TYPE_BOOL",
                Type::String => "TYPE_STRING",
                Type::Group => "TYPE_GROUP",
                Type::Message => "TYPE_MESSAGE",
                Type::Bytes => "TYPE_BYTES",
                Type::Uint32 => "TYPE_UINT32",
                Type::Enum => "TYPE_ENUM",
                Type::Sfixed32 => "TYPE_SFIXED32",
                Type::Sfixed64 => "TYPE_SFIXED64",
                Type::Sint32 => "TYPE_SINT32",
                Type::Sint64 => "TYPE_SINT64",
            }
        }
    }
    #[repr(i32)]
    pub enum Label {
        /// 0 is reserved for errors
        Optional = 1,
        Required = 2,
        Repeated = 3,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Label {
        #[inline]
        fn clone(&self) -> Label {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Label {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Label {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Label::Optional => ::core::fmt::Formatter::write_str(f, "Optional"),
                Label::Required => ::core::fmt::Formatter::write_str(f, "Required"),
                Label::Repeated => ::core::fmt::Formatter::write_str(f, "Repeated"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Label {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Label {
        #[inline]
        fn eq(&self, other: &Label) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Label {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Label {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Label {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Label {
        #[inline]
        fn partial_cmp(&self, other: &Label) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Label {
        #[inline]
        fn cmp(&self, other: &Label) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    impl Label {
        ///Returns `true` if `value` is a variant of `Label`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                1 => true,
                2 => true,
                3 => true,
                _ => false,
            }
        }
        ///Converts an `i32` to a `Label`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<Label> {
            match value {
                1 => ::core::option::Option::Some(Label::Optional),
                2 => ::core::option::Option::Some(Label::Required),
                3 => ::core::option::Option::Some(Label::Repeated),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for Label {
        fn default() -> Label {
            Label::Optional
        }
    }
    impl ::core::convert::From<Label> for i32 {
        fn from(value: Label) -> i32 {
            value as i32
        }
    }
    impl Label {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Label::Optional => "LABEL_OPTIONAL",
                Label::Required => "LABEL_REQUIRED",
                Label::Repeated => "LABEL_REPEATED",
            }
        }
    }
}
/// Describes a oneof.
pub struct OneofDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<OneofOptions>,
}
#[automatically_derived]
impl ::core::clone::Clone for OneofDescriptorProto {
    #[inline]
    fn clone(&self) -> OneofDescriptorProto {
        OneofDescriptorProto {
            name: ::core::clone::Clone::clone(&self.name),
            options: ::core::clone::Clone::clone(&self.options),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OneofDescriptorProto {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OneofDescriptorProto {
    #[inline]
    fn eq(&self, other: &OneofDescriptorProto) -> bool {
        self.name == other.name && self.options == other.options
    }
}
impl ::prost::Message for OneofDescriptorProto {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.name, buf);
        if let Some(ref msg) = self.options {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "OneofDescriptorProto";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "options");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + self
            .options
            .as_ref()
            .map_or(0, |msg| ::prost::encoding::message::encoded_len(2u32, msg))
    }
    fn clear(&mut self) {
        self.name = ::core::option::Option::None;
        self.options = ::core::option::Option::None;
    }
}
impl ::core::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        OneofDescriptorProto {
            name: ::core::option::Option::None,
            options: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for OneofDescriptorProto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("OneofDescriptorProto");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl OneofDescriptorProto {
    ///Returns the value of `name`, or the default value if `name` is unset.
    pub fn name(&self) -> &str {
        match self.name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
}
/// Describes an enum type.
pub struct EnumDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub value: ::prost::alloc::vec::Vec<EnumValueDescriptorProto>,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<EnumOptions>,
    /// Range of reserved numeric values. Reserved numeric values may not be used
    /// by enum values in the same enum declaration. Reserved ranges may not
    /// overlap.
    #[prost(message, repeated, tag = "4")]
    pub reserved_range: ::prost::alloc::vec::Vec<enum_descriptor_proto::EnumReservedRange>,
    /// Reserved enum value names, which may not be reused. A given name may only
    /// be reserved once.
    #[prost(string, repeated, tag = "5")]
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[automatically_derived]
impl ::core::clone::Clone for EnumDescriptorProto {
    #[inline]
    fn clone(&self) -> EnumDescriptorProto {
        EnumDescriptorProto {
            name: ::core::clone::Clone::clone(&self.name),
            value: ::core::clone::Clone::clone(&self.value),
            options: ::core::clone::Clone::clone(&self.options),
            reserved_range: ::core::clone::Clone::clone(&self.reserved_range),
            reserved_name: ::core::clone::Clone::clone(&self.reserved_name),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumDescriptorProto {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumDescriptorProto {
    #[inline]
    fn eq(&self, other: &EnumDescriptorProto) -> bool {
        self.name == other.name
            && self.value == other.value
            && self.options == other.options
            && self.reserved_range == other.reserved_range
            && self.reserved_name == other.reserved_name
    }
}
impl ::prost::Message for EnumDescriptorProto {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.name, buf);
        for msg in &self.value {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
        if let Some(ref msg) = self.options {
            ::prost::encoding::message::encode(3u32, msg, buf);
        }
        for msg in &self.reserved_range {
            ::prost::encoding::message::encode(4u32, msg, buf);
        }
        <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(5u32, &self.reserved_name, buf);
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "EnumDescriptorProto";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.value;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "value");
                        error
                    },
                )
            }
            3u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "options");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.reserved_range;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "reserved_range");
                        error
                    },
                )
            }
            5u32 => {
                let mut value = &mut self.reserved_name;
                <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "reserved_name");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + ::prost::encoding::message::encoded_len_repeated(2u32, &self.value)
            + self
                .options
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(3u32, msg))
            + ::prost::encoding::message::encoded_len_repeated(4u32, &self.reserved_range)
            + ::prost::encoding::string::encoded_len_repeated(5u32, &self.reserved_name)
    }
    fn clear(&mut self) {
        self.name = ::core::option::Option::None;
        self.value.clear();
        self.options = ::core::option::Option::None;
        self.reserved_range.clear();
        self.reserved_name.clear();
    }
}
impl ::core::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        EnumDescriptorProto {
            name: ::core::option::Option::None,
            value: ::core::default::Default::default(),
            options: ::core::default::Default::default(),
            reserved_range: ::core::default::Default::default(),
            reserved_name: ::prost::alloc::vec::Vec::new(),
        }
    }
}
impl ::core::fmt::Debug for EnumDescriptorProto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("EnumDescriptorProto");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = &self.value;
            builder.field("value", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = &self.reserved_range;
            builder.field("reserved_range", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let mut vec_builder = f.debug_list();
                        for v in self.0 {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            vec_builder.entry(&Inner(v));
                        }
                        vec_builder.finish()
                    }
                }
                ScalarWrapper(&self.reserved_name)
            };
            builder.field("reserved_name", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl EnumDescriptorProto {
    ///Returns the value of `name`, or the default value if `name` is unset.
    pub fn name(&self) -> &str {
        match self.name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
}
/// Nested message and enum types in `EnumDescriptorProto`.
pub mod enum_descriptor_proto {
    /// Range of reserved numeric values. Reserved values may not be used by
    /// entries in the same enum. Reserved ranges may not overlap.
    ///
    /// Note that this is distinct from DescriptorProto.ReservedRange in that it
    /// is inclusive such that it can appropriately represent the entire int32
    /// domain.
    pub struct EnumReservedRange {
        /// Inclusive.
        #[prost(int32, optional, tag = "1")]
        pub start: ::core::option::Option<i32>,
        /// Inclusive.
        #[prost(int32, optional, tag = "2")]
        pub end: ::core::option::Option<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EnumReservedRange {
        #[inline]
        fn clone(&self) -> EnumReservedRange {
            EnumReservedRange {
                start: ::core::clone::Clone::clone(&self.start),
                end: ::core::clone::Clone::clone(&self.end),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumReservedRange {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumReservedRange {
        #[inline]
        fn eq(&self, other: &EnumReservedRange) -> bool {
            self.start == other.start && self.end == other.end
        }
    }
    impl ::prost::Message for EnumReservedRange {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(1u32, &self.start, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(2u32, &self.end, buf);
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "EnumReservedRange";
            match tag {
                1u32 => {
                    let mut value = &mut self.start;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "start");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.end;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "end");
                        error
                    })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.start.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(1u32, value)
            }) + self.end.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(2u32, value)
            })
        }
        fn clear(&mut self) {
            self.start = ::core::option::Option::None;
            self.end = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for EnumReservedRange {
        fn default() -> Self {
            EnumReservedRange {
                start: ::core::option::Option::None,
                end: ::core::option::Option::None,
            }
        }
    }
    impl ::core::fmt::Debug for EnumReservedRange {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("EnumReservedRange");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.start)
                };
                builder.field("start", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.end)
                };
                builder.field("end", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl EnumReservedRange {
        ///Returns the value of `start`, or the default value if `start` is unset.
        pub fn start(&self) -> i32 {
            match self.start {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `end`, or the default value if `end` is unset.
        pub fn end(&self) -> i32 {
            match self.end {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
    }
}
/// Describes a value within an enum.
pub struct EnumValueDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub number: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<EnumValueOptions>,
}
#[automatically_derived]
impl ::core::clone::Clone for EnumValueDescriptorProto {
    #[inline]
    fn clone(&self) -> EnumValueDescriptorProto {
        EnumValueDescriptorProto {
            name: ::core::clone::Clone::clone(&self.name),
            number: ::core::clone::Clone::clone(&self.number),
            options: ::core::clone::Clone::clone(&self.options),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumValueDescriptorProto {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumValueDescriptorProto {
    #[inline]
    fn eq(&self, other: &EnumValueDescriptorProto) -> bool {
        self.name == other.name && self.number == other.number && self.options == other.options
    }
}
impl ::prost::Message for EnumValueDescriptorProto {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.name, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::Int32,
            _,
            _,
        >(2u32, &self.number, buf);
        if let Some(ref msg) = self.options {
            ::prost::encoding::message::encode(3u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "EnumValueDescriptorProto";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.number;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "number");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "options");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + self.number.as_ref().map_or(0, |value| {
            ::prost::encoding::int32::encoded_len(2u32, value)
        }) + self
            .options
            .as_ref()
            .map_or(0, |msg| ::prost::encoding::message::encoded_len(3u32, msg))
    }
    fn clear(&mut self) {
        self.name = ::core::option::Option::None;
        self.number = ::core::option::Option::None;
        self.options = ::core::option::Option::None;
    }
}
impl ::core::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        EnumValueDescriptorProto {
            name: ::core::option::Option::None,
            number: ::core::option::Option::None,
            options: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for EnumValueDescriptorProto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("EnumValueDescriptorProto");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.number)
            };
            builder.field("number", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl EnumValueDescriptorProto {
    ///Returns the value of `name`, or the default value if `name` is unset.
    pub fn name(&self) -> &str {
        match self.name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `number`, or the default value if `number` is unset.
    pub fn number(&self) -> i32 {
        match self.number {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => 0i32,
        }
    }
}
/// Describes a service.
pub struct ServiceDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub method: ::prost::alloc::vec::Vec<MethodDescriptorProto>,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<ServiceOptions>,
}
#[automatically_derived]
impl ::core::clone::Clone for ServiceDescriptorProto {
    #[inline]
    fn clone(&self) -> ServiceDescriptorProto {
        ServiceDescriptorProto {
            name: ::core::clone::Clone::clone(&self.name),
            method: ::core::clone::Clone::clone(&self.method),
            options: ::core::clone::Clone::clone(&self.options),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ServiceDescriptorProto {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ServiceDescriptorProto {
    #[inline]
    fn eq(&self, other: &ServiceDescriptorProto) -> bool {
        self.name == other.name && self.method == other.method && self.options == other.options
    }
}
impl ::prost::Message for ServiceDescriptorProto {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.name, buf);
        for msg in &self.method {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
        if let Some(ref msg) = self.options {
            ::prost::encoding::message::encode(3u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "ServiceDescriptorProto";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.method;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "method");
                        error
                    },
                )
            }
            3u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "options");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + ::prost::encoding::message::encoded_len_repeated(2u32, &self.method)
            + self
                .options
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(3u32, msg))
    }
    fn clear(&mut self) {
        self.name = ::core::option::Option::None;
        self.method.clear();
        self.options = ::core::option::Option::None;
    }
}
impl ::core::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        ServiceDescriptorProto {
            name: ::core::option::Option::None,
            method: ::core::default::Default::default(),
            options: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for ServiceDescriptorProto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("ServiceDescriptorProto");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = &self.method;
            builder.field("method", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl ServiceDescriptorProto {
    ///Returns the value of `name`, or the default value if `name` is unset.
    pub fn name(&self) -> &str {
        match self.name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
}
/// Describes a method of a service.
pub struct MethodDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Input and output type names.  These are resolved in the same way as
    /// FieldDescriptorProto.type_name, but must refer to a message type.
    #[prost(string, optional, tag = "2")]
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub options: ::core::option::Option<MethodOptions>,
    /// Identifies if client streams multiple client messages
    #[prost(bool, optional, tag = "5", default = "false")]
    pub client_streaming: ::core::option::Option<bool>,
    /// Identifies if server streams multiple server messages
    #[prost(bool, optional, tag = "6", default = "false")]
    pub server_streaming: ::core::option::Option<bool>,
}
#[automatically_derived]
impl ::core::clone::Clone for MethodDescriptorProto {
    #[inline]
    fn clone(&self) -> MethodDescriptorProto {
        MethodDescriptorProto {
            name: ::core::clone::Clone::clone(&self.name),
            input_type: ::core::clone::Clone::clone(&self.input_type),
            output_type: ::core::clone::Clone::clone(&self.output_type),
            options: ::core::clone::Clone::clone(&self.options),
            client_streaming: ::core::clone::Clone::clone(&self.client_streaming),
            server_streaming: ::core::clone::Clone::clone(&self.server_streaming),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MethodDescriptorProto {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MethodDescriptorProto {
    #[inline]
    fn eq(&self, other: &MethodDescriptorProto) -> bool {
        self.name == other.name
            && self.input_type == other.input_type
            && self.output_type == other.output_type
            && self.options == other.options
            && self.client_streaming == other.client_streaming
            && self.server_streaming == other.server_streaming
    }
}
impl ::prost::Message for MethodDescriptorProto {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.name, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(2u32, &self.input_type, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(3u32, &self.output_type, buf);
        if let Some(ref msg) = self.options {
            ::prost::encoding::message::encode(4u32, msg, buf);
        }
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(5u32, &self.client_streaming, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(6u32, &self.server_streaming, buf);
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "MethodDescriptorProto";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.input_type;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "input_type");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.output_type;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "output_type");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "options");
                    error
                })
            }
            5u32 => {
                let mut value = &mut self.client_streaming;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "client_streaming");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.server_streaming;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "server_streaming");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.name.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + self.input_type.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(2u32, value)
        }) + self.output_type.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(3u32, value)
        }) + self
            .options
            .as_ref()
            .map_or(0, |msg| ::prost::encoding::message::encoded_len(4u32, msg))
            + self
                .client_streaming
                .as_ref()
                .map_or(0, |value| ::prost::encoding::bool::encoded_len(5u32, value))
            + self
                .server_streaming
                .as_ref()
                .map_or(0, |value| ::prost::encoding::bool::encoded_len(6u32, value))
    }
    fn clear(&mut self) {
        self.name = ::core::option::Option::None;
        self.input_type = ::core::option::Option::None;
        self.output_type = ::core::option::Option::None;
        self.options = ::core::option::Option::None;
        self.client_streaming = ::core::option::Option::None;
        self.server_streaming = ::core::option::Option::None;
    }
}
impl ::core::default::Default for MethodDescriptorProto {
    fn default() -> Self {
        MethodDescriptorProto {
            name: ::core::option::Option::None,
            input_type: ::core::option::Option::None,
            output_type: ::core::option::Option::None,
            options: ::core::default::Default::default(),
            client_streaming: ::core::option::Option::None,
            server_streaming: ::core::option::Option::None,
        }
    }
}
impl ::core::fmt::Debug for MethodDescriptorProto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("MethodDescriptorProto");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.input_type)
            };
            builder.field("input_type", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.output_type)
            };
            builder.field("output_type", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.client_streaming)
            };
            builder.field("client_streaming", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.server_streaming)
            };
            builder.field("server_streaming", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl MethodDescriptorProto {
    ///Returns the value of `name`, or the default value if `name` is unset.
    pub fn name(&self) -> &str {
        match self.name {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `input_type`, or the default value if `input_type` is unset.
    pub fn input_type(&self) -> &str {
        match self.input_type {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `output_type`, or the default value if `output_type` is unset.
    pub fn output_type(&self) -> &str {
        match self.output_type {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `client_streaming`, or the default value if `client_streaming` is unset.
    pub fn client_streaming(&self) -> bool {
        match self.client_streaming {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `server_streaming`, or the default value if `server_streaming` is unset.
    pub fn server_streaming(&self) -> bool {
        match self.server_streaming {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
}
/// Each of the definitions above may have "options" attached.  These are
/// just annotations which may cause code to be generated slightly differently
/// or may contain hints for code that manipulates protocol messages.
///
/// Clients may define custom options as extensions of the \*Options messages.
/// These extensions may not yet be known at parsing time, so the parser cannot
/// store the values in them.  Instead it stores them in a field in the \*Options
/// message called uninterpreted_option. This field must have the same name
/// across all \*Options messages. We then use this field to populate the
/// extensions when we build a descriptor, at which point all protos have been
/// parsed and so all extensions are known.
///
/// Extension numbers for custom options may be chosen as follows:
///
/// * For options which will only be used within a single application or
///   organization, or for experimental options, use field numbers 50000
///   through 99999.  It is up to you to ensure that you do not use the
///   same number for multiple options.
/// * For options which will be published and used publicly by multiple
///   independent entities, e-mail protobuf-global-extension-registry@google.com
///   to reserve extension numbers. Simply provide your project name (e.g.
///   Objective-C plugin) and your project website (if available) -- there's no
///   need to explain how you intend to use them. Usually you only need one
///   extension number. You can declare multiple options with only one extension
///   number by putting them in a sub-message. See the Custom Options section of
///   the docs for examples:
///   <https://developers.google.com/protocol-buffers/docs/proto#options>
///   If this turns out to be popular, a web service will be set up
///   to automatically assign option numbers.
pub struct FileOptions {
    /// Sets the Java package where classes generated from this .proto will be
    /// placed.  By default, the proto package is used, but this is often
    /// inappropriate because proto packages do not normally start with backwards
    /// domain names.
    #[prost(string, optional, tag = "1")]
    pub java_package: ::core::option::Option<::prost::alloc::string::String>,
    /// Controls the name of the wrapper Java class generated for the .proto file.
    /// That class will always contain the .proto file's getDescriptor() method as
    /// well as any top-level extensions defined in the .proto file.
    /// If java_multiple_files is disabled, then all the other classes from the
    /// .proto file will be nested inside the single wrapper outer class.
    #[prost(string, optional, tag = "8")]
    pub java_outer_classname: ::core::option::Option<::prost::alloc::string::String>,
    /// If enabled, then the Java code generator will generate a separate .java
    /// file for each top-level message, enum, and service defined in the .proto
    /// file.  Thus, these types will *not* be nested inside the wrapper class
    /// named by java_outer_classname.  However, the wrapper class will still be
    /// generated to contain the file's getDescriptor() method as well as any
    /// top-level extensions defined in the file.
    #[prost(bool, optional, tag = "10", default = "false")]
    pub java_multiple_files: ::core::option::Option<bool>,
    /// This option does nothing.
    #[deprecated]
    #[prost(bool, optional, tag = "20")]
    pub java_generate_equals_and_hash: ::core::option::Option<bool>,
    /// If set true, then the Java2 code generator will generate code that
    /// throws an exception whenever an attempt is made to assign a non-UTF-8
    /// byte sequence to a string field.
    /// Message reflection will do the same.
    /// However, an extension field still accepts non-UTF-8 byte sequences.
    /// This option has no effect on when used with the lite runtime.
    #[prost(bool, optional, tag = "27", default = "false")]
    pub java_string_check_utf8: ::core::option::Option<bool>,
    #[prost(
        enumeration = "file_options::OptimizeMode",
        optional,
        tag = "9",
        default = "Speed"
    )]
    pub optimize_for: ::core::option::Option<i32>,
    /// Sets the Go package where structs generated from this .proto will be
    /// placed. If omitted, the Go package will be derived from the following:
    ///
    /// * The basename of the package import path, if provided.
    /// * Otherwise, the package statement in the .proto file, if present.
    /// * Otherwise, the basename of the .proto file, without extension.
    #[prost(string, optional, tag = "11")]
    pub go_package: ::core::option::Option<::prost::alloc::string::String>,
    /// Should generic services be generated in each language?  "Generic" services
    /// are not specific to any particular RPC system.  They are generated by the
    /// main code generators in each language (without additional plugins).
    /// Generic services were the only kind of service generation supported by
    /// early versions of google.protobuf.
    ///
    /// Generic services are now considered deprecated in favor of using plugins
    /// that generate code specific to your particular RPC system.  Therefore,
    /// these default to false.  Old code which depends on generic services should
    /// explicitly set them to true.
    #[prost(bool, optional, tag = "16", default = "false")]
    pub cc_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "17", default = "false")]
    pub java_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "18", default = "false")]
    pub py_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "42", default = "false")]
    pub php_generic_services: ::core::option::Option<bool>,
    /// Is this file deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for everything in the file, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating files.
    #[prost(bool, optional, tag = "23", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    /// Enables the use of arenas for the proto messages in this file. This applies
    /// only to generated classes for C++.
    #[prost(bool, optional, tag = "31", default = "true")]
    pub cc_enable_arenas: ::core::option::Option<bool>,
    /// Sets the objective c class prefix which is prepended to all objective c
    /// generated classes from this .proto. There is no default.
    #[prost(string, optional, tag = "36")]
    pub objc_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace for generated classes; defaults to the package.
    #[prost(string, optional, tag = "37")]
    pub csharp_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// By default Swift generators will take the proto package and CamelCase it
    /// replacing '.' with underscore and use that to prefix the types/symbols
    /// defined. When this options is provided, they will use this value instead
    /// to prefix the types/symbols defined.
    #[prost(string, optional, tag = "39")]
    pub swift_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Sets the php class prefix which is prepended to all php generated classes
    /// from this .proto. Default is empty.
    #[prost(string, optional, tag = "40")]
    pub php_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the namespace of php generated classes. Default
    /// is empty. When this option is empty, the package name will be used for
    /// determining the namespace.
    #[prost(string, optional, tag = "41")]
    pub php_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the namespace of php generated metadata classes.
    /// Default is empty. When this option is empty, the proto file name will be
    /// used for determining the namespace.
    #[prost(string, optional, tag = "44")]
    pub php_metadata_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the package of ruby generated classes. Default
    /// is empty. When this option is not set, the package name will be used for
    /// determining the ruby package.
    #[prost(string, optional, tag = "45")]
    pub ruby_package: ::core::option::Option<::prost::alloc::string::String>,
    /// The parser stores options it doesn't recognize here.
    /// See the documentation for the "Options" section above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for FileOptions {
    #[inline]
    fn clone(&self) -> FileOptions {
        FileOptions {
            java_package: ::core::clone::Clone::clone(&self.java_package),
            java_outer_classname: ::core::clone::Clone::clone(&self.java_outer_classname),
            java_multiple_files: ::core::clone::Clone::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: ::core::clone::Clone::clone(
                &self.java_generate_equals_and_hash,
            ),
            java_string_check_utf8: ::core::clone::Clone::clone(&self.java_string_check_utf8),
            optimize_for: ::core::clone::Clone::clone(&self.optimize_for),
            go_package: ::core::clone::Clone::clone(&self.go_package),
            cc_generic_services: ::core::clone::Clone::clone(&self.cc_generic_services),
            java_generic_services: ::core::clone::Clone::clone(&self.java_generic_services),
            py_generic_services: ::core::clone::Clone::clone(&self.py_generic_services),
            php_generic_services: ::core::clone::Clone::clone(&self.php_generic_services),
            deprecated: ::core::clone::Clone::clone(&self.deprecated),
            cc_enable_arenas: ::core::clone::Clone::clone(&self.cc_enable_arenas),
            objc_class_prefix: ::core::clone::Clone::clone(&self.objc_class_prefix),
            csharp_namespace: ::core::clone::Clone::clone(&self.csharp_namespace),
            swift_prefix: ::core::clone::Clone::clone(&self.swift_prefix),
            php_class_prefix: ::core::clone::Clone::clone(&self.php_class_prefix),
            php_namespace: ::core::clone::Clone::clone(&self.php_namespace),
            php_metadata_namespace: ::core::clone::Clone::clone(&self.php_metadata_namespace),
            ruby_package: ::core::clone::Clone::clone(&self.ruby_package),
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FileOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FileOptions {
    #[inline]
    fn eq(&self, other: &FileOptions) -> bool {
        self.java_package == other.java_package
            && self.java_outer_classname == other.java_outer_classname
            && self.java_multiple_files == other.java_multiple_files
            && self.java_generate_equals_and_hash == other.java_generate_equals_and_hash
            && self.java_string_check_utf8 == other.java_string_check_utf8
            && self.optimize_for == other.optimize_for
            && self.go_package == other.go_package
            && self.cc_generic_services == other.cc_generic_services
            && self.java_generic_services == other.java_generic_services
            && self.py_generic_services == other.py_generic_services
            && self.php_generic_services == other.php_generic_services
            && self.deprecated == other.deprecated
            && self.cc_enable_arenas == other.cc_enable_arenas
            && self.objc_class_prefix == other.objc_class_prefix
            && self.csharp_namespace == other.csharp_namespace
            && self.swift_prefix == other.swift_prefix
            && self.php_class_prefix == other.php_class_prefix
            && self.php_namespace == other.php_namespace
            && self.php_metadata_namespace == other.php_metadata_namespace
            && self.ruby_package == other.ruby_package
            && self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for FileOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.java_package, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(8u32, &self.java_outer_classname, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufEnum,
            _,
            _,
        >(9u32, &self.optimize_for, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(10u32, &self.java_multiple_files, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(11u32, &self.go_package, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(16u32, &self.cc_generic_services, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(17u32, &self.java_generic_services, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(18u32, &self.py_generic_services, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(20u32, &self.java_generate_equals_and_hash, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(23u32, &self.deprecated, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(27u32, &self.java_string_check_utf8, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(31u32, &self.cc_enable_arenas, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(36u32, &self.objc_class_prefix, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(37u32, &self.csharp_namespace, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(39u32, &self.swift_prefix, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(40u32, &self.php_class_prefix, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(41u32, &self.php_namespace, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(42u32, &self.php_generic_services, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(44u32, &self.php_metadata_namespace, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(45u32, &self.ruby_package, buf);
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "FileOptions";
        match tag {
            1u32 => {
                let mut value = &mut self.java_package;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "java_package");
                    error
                })
            }
            8u32 => {
                let mut value = &mut self.java_outer_classname;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "java_outer_classname");
                    error
                })
            }
            9u32 => {
                let mut value = &mut self.optimize_for;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "optimize_for");
                    error
                })
            }
            10u32 => {
                let mut value = &mut self.java_multiple_files;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "java_multiple_files");
                    error
                })
            }
            11u32 => {
                let mut value = &mut self.go_package;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "go_package");
                    error
                })
            }
            16u32 => {
                let mut value = &mut self.cc_generic_services;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "cc_generic_services");
                    error
                })
            }
            17u32 => {
                let mut value = &mut self.java_generic_services;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "java_generic_services");
                    error
                })
            }
            18u32 => {
                let mut value = &mut self.py_generic_services;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "py_generic_services");
                    error
                })
            }
            20u32 => {
                let mut value = &mut self.java_generate_equals_and_hash;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "java_generate_equals_and_hash");
                    error
                })
            }
            23u32 => {
                let mut value = &mut self.deprecated;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "deprecated");
                    error
                })
            }
            27u32 => {
                let mut value = &mut self.java_string_check_utf8;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "java_string_check_utf8");
                    error
                })
            }
            31u32 => {
                let mut value = &mut self.cc_enable_arenas;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "cc_enable_arenas");
                    error
                })
            }
            36u32 => {
                let mut value = &mut self.objc_class_prefix;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "objc_class_prefix");
                    error
                })
            }
            37u32 => {
                let mut value = &mut self.csharp_namespace;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "csharp_namespace");
                    error
                })
            }
            39u32 => {
                let mut value = &mut self.swift_prefix;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "swift_prefix");
                    error
                })
            }
            40u32 => {
                let mut value = &mut self.php_class_prefix;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "php_class_prefix");
                    error
                })
            }
            41u32 => {
                let mut value = &mut self.php_namespace;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "php_namespace");
                    error
                })
            }
            42u32 => {
                let mut value = &mut self.php_generic_services;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "php_generic_services");
                    error
                })
            }
            44u32 => {
                let mut value = &mut self.php_metadata_namespace;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "php_metadata_namespace");
                    error
                })
            }
            45u32 => {
                let mut value = &mut self.ruby_package;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "ruby_package");
                    error
                })
            }
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.java_package.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(1u32, value)
        }) + self.java_outer_classname.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(8u32, value)
        }) + self.optimize_for.as_ref().map_or(0, |value| {
            ::prost::encoding::int32::encoded_len(9u32, value)
        }) + self.java_multiple_files.as_ref().map_or(0, |value| {
            ::prost::encoding::bool::encoded_len(10u32, value)
        }) + self.go_package.as_ref().map_or(0, |value| {
            ::prost::encoding::string::encoded_len(11u32, value)
        }) + self.cc_generic_services.as_ref().map_or(0, |value| {
            ::prost::encoding::bool::encoded_len(16u32, value)
        }) + self.java_generic_services.as_ref().map_or(0, |value| {
            ::prost::encoding::bool::encoded_len(17u32, value)
        }) + self.py_generic_services.as_ref().map_or(0, |value| {
            ::prost::encoding::bool::encoded_len(18u32, value)
        }) + self
            .java_generate_equals_and_hash
            .as_ref()
            .map_or(0, |value| {
                ::prost::encoding::bool::encoded_len(20u32, value)
            })
            + self.deprecated.as_ref().map_or(0, |value| {
                ::prost::encoding::bool::encoded_len(23u32, value)
            })
            + self.java_string_check_utf8.as_ref().map_or(0, |value| {
                ::prost::encoding::bool::encoded_len(27u32, value)
            })
            + self.cc_enable_arenas.as_ref().map_or(0, |value| {
                ::prost::encoding::bool::encoded_len(31u32, value)
            })
            + self.objc_class_prefix.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(36u32, value)
            })
            + self.csharp_namespace.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(37u32, value)
            })
            + self.swift_prefix.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(39u32, value)
            })
            + self.php_class_prefix.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(40u32, value)
            })
            + self.php_namespace.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(41u32, value)
            })
            + self.php_generic_services.as_ref().map_or(0, |value| {
                ::prost::encoding::bool::encoded_len(42u32, value)
            })
            + self.php_metadata_namespace.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(44u32, value)
            })
            + self.ruby_package.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(45u32, value)
            })
            + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.java_package = ::core::option::Option::None;
        self.java_outer_classname = ::core::option::Option::None;
        self.optimize_for = ::core::option::Option::None;
        self.java_multiple_files = ::core::option::Option::None;
        self.go_package = ::core::option::Option::None;
        self.cc_generic_services = ::core::option::Option::None;
        self.java_generic_services = ::core::option::Option::None;
        self.py_generic_services = ::core::option::Option::None;
        self.java_generate_equals_and_hash = ::core::option::Option::None;
        self.deprecated = ::core::option::Option::None;
        self.java_string_check_utf8 = ::core::option::Option::None;
        self.cc_enable_arenas = ::core::option::Option::None;
        self.objc_class_prefix = ::core::option::Option::None;
        self.csharp_namespace = ::core::option::Option::None;
        self.swift_prefix = ::core::option::Option::None;
        self.php_class_prefix = ::core::option::Option::None;
        self.php_namespace = ::core::option::Option::None;
        self.php_generic_services = ::core::option::Option::None;
        self.php_metadata_namespace = ::core::option::Option::None;
        self.ruby_package = ::core::option::Option::None;
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for FileOptions {
    fn default() -> Self {
        FileOptions {
            java_package: ::core::option::Option::None,
            java_outer_classname: ::core::option::Option::None,
            optimize_for: ::core::option::Option::None,
            java_multiple_files: ::core::option::Option::None,
            go_package: ::core::option::Option::None,
            cc_generic_services: ::core::option::Option::None,
            java_generic_services: ::core::option::Option::None,
            py_generic_services: ::core::option::Option::None,
            java_generate_equals_and_hash: ::core::option::Option::None,
            deprecated: ::core::option::Option::None,
            java_string_check_utf8: ::core::option::Option::None,
            cc_enable_arenas: ::core::option::Option::None,
            objc_class_prefix: ::core::option::Option::None,
            csharp_namespace: ::core::option::Option::None,
            swift_prefix: ::core::option::Option::None,
            php_class_prefix: ::core::option::Option::None,
            php_namespace: ::core::option::Option::None,
            php_generic_services: ::core::option::Option::None,
            php_metadata_namespace: ::core::option::Option::None,
            ruby_package: ::core::option::Option::None,
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for FileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("FileOptions");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.java_package)
            };
            builder.field("java_package", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.java_outer_classname)
            };
            builder.field("java_outer_classname", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.java_multiple_files)
            };
            builder.field("java_multiple_files", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.java_generate_equals_and_hash)
            };
            builder.field("java_generate_equals_and_hash", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.java_string_check_utf8)
            };
            builder.field("java_string_check_utf8", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        struct Inner<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for Inner<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                match file_options::OptimizeMode::from_i32(*self.0) {
                                    None => ::core::fmt::Debug::fmt(&self.0, f),
                                    Some(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.optimize_for)
            };
            builder.field("optimize_for", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.go_package)
            };
            builder.field("go_package", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.cc_generic_services)
            };
            builder.field("cc_generic_services", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.java_generic_services)
            };
            builder.field("java_generic_services", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.py_generic_services)
            };
            builder.field("py_generic_services", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.php_generic_services)
            };
            builder.field("php_generic_services", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.deprecated)
            };
            builder.field("deprecated", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.cc_enable_arenas)
            };
            builder.field("cc_enable_arenas", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.objc_class_prefix)
            };
            builder.field("objc_class_prefix", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.csharp_namespace)
            };
            builder.field("csharp_namespace", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.swift_prefix)
            };
            builder.field("swift_prefix", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.php_class_prefix)
            };
            builder.field("php_class_prefix", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.php_namespace)
            };
            builder.field("php_namespace", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.php_metadata_namespace)
            };
            builder.field("php_metadata_namespace", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.ruby_package)
            };
            builder.field("ruby_package", &wrapper)
        };
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl FileOptions {
    ///Returns the value of `java_package`, or the default value if `java_package` is unset.
    pub fn java_package(&self) -> &str {
        match self.java_package {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `java_outer_classname`, or the default value if `java_outer_classname` is unset.
    pub fn java_outer_classname(&self) -> &str {
        match self.java_outer_classname {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the enum value of `optimize_for`, or the default if the field is unset or set to an invalid enum value.
    pub fn optimize_for(&self) -> file_options::OptimizeMode {
        self.optimize_for
            .and_then(file_options::OptimizeMode::from_i32)
            .unwrap_or(file_options::OptimizeMode::Speed)
    }
    ///Sets `optimize_for` to the provided enum value.
    pub fn set_optimize_for(&mut self, value: file_options::OptimizeMode) {
        self.optimize_for = ::core::option::Option::Some(value as i32);
    }
    ///Returns the value of `java_multiple_files`, or the default value if `java_multiple_files` is unset.
    pub fn java_multiple_files(&self) -> bool {
        match self.java_multiple_files {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `go_package`, or the default value if `go_package` is unset.
    pub fn go_package(&self) -> &str {
        match self.go_package {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `cc_generic_services`, or the default value if `cc_generic_services` is unset.
    pub fn cc_generic_services(&self) -> bool {
        match self.cc_generic_services {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `java_generic_services`, or the default value if `java_generic_services` is unset.
    pub fn java_generic_services(&self) -> bool {
        match self.java_generic_services {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `py_generic_services`, or the default value if `py_generic_services` is unset.
    pub fn py_generic_services(&self) -> bool {
        match self.py_generic_services {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `java_generate_equals_and_hash`, or the default value if `java_generate_equals_and_hash` is unset.
    pub fn java_generate_equals_and_hash(&self) -> bool {
        match self.java_generate_equals_and_hash {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `deprecated`, or the default value if `deprecated` is unset.
    pub fn deprecated(&self) -> bool {
        match self.deprecated {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `java_string_check_utf8`, or the default value if `java_string_check_utf8` is unset.
    pub fn java_string_check_utf8(&self) -> bool {
        match self.java_string_check_utf8 {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `cc_enable_arenas`, or the default value if `cc_enable_arenas` is unset.
    pub fn cc_enable_arenas(&self) -> bool {
        match self.cc_enable_arenas {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => true,
        }
    }
    ///Returns the value of `objc_class_prefix`, or the default value if `objc_class_prefix` is unset.
    pub fn objc_class_prefix(&self) -> &str {
        match self.objc_class_prefix {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `csharp_namespace`, or the default value if `csharp_namespace` is unset.
    pub fn csharp_namespace(&self) -> &str {
        match self.csharp_namespace {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `swift_prefix`, or the default value if `swift_prefix` is unset.
    pub fn swift_prefix(&self) -> &str {
        match self.swift_prefix {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `php_class_prefix`, or the default value if `php_class_prefix` is unset.
    pub fn php_class_prefix(&self) -> &str {
        match self.php_class_prefix {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `php_namespace`, or the default value if `php_namespace` is unset.
    pub fn php_namespace(&self) -> &str {
        match self.php_namespace {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `php_generic_services`, or the default value if `php_generic_services` is unset.
    pub fn php_generic_services(&self) -> bool {
        match self.php_generic_services {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `php_metadata_namespace`, or the default value if `php_metadata_namespace` is unset.
    pub fn php_metadata_namespace(&self) -> &str {
        match self.php_metadata_namespace {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `ruby_package`, or the default value if `ruby_package` is unset.
    pub fn ruby_package(&self) -> &str {
        match self.ruby_package {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
}
/// Nested message and enum types in `FileOptions`.
pub mod file_options {
    /// Generated classes can be optimized for speed or code size.
    #[repr(i32)]
    pub enum OptimizeMode {
        /// Generate complete code for parsing, serialization,
        Speed = 1,
        /// etc.
        ///
        /// Use ReflectionOps to implement these methods.
        CodeSize = 2,
        /// Generate code using MessageLite and the lite runtime.
        LiteRuntime = 3,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OptimizeMode {
        #[inline]
        fn clone(&self) -> OptimizeMode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for OptimizeMode {}
    #[automatically_derived]
    impl ::core::fmt::Debug for OptimizeMode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                OptimizeMode::Speed => ::core::fmt::Formatter::write_str(f, "Speed"),
                OptimizeMode::CodeSize => ::core::fmt::Formatter::write_str(f, "CodeSize"),
                OptimizeMode::LiteRuntime => ::core::fmt::Formatter::write_str(f, "LiteRuntime"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for OptimizeMode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for OptimizeMode {
        #[inline]
        fn eq(&self, other: &OptimizeMode) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for OptimizeMode {}
    #[automatically_derived]
    impl ::core::cmp::Eq for OptimizeMode {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for OptimizeMode {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for OptimizeMode {
        #[inline]
        fn partial_cmp(
            &self,
            other: &OptimizeMode,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for OptimizeMode {
        #[inline]
        fn cmp(&self, other: &OptimizeMode) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    impl OptimizeMode {
        ///Returns `true` if `value` is a variant of `OptimizeMode`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                1 => true,
                2 => true,
                3 => true,
                _ => false,
            }
        }
        ///Converts an `i32` to a `OptimizeMode`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<OptimizeMode> {
            match value {
                1 => ::core::option::Option::Some(OptimizeMode::Speed),
                2 => ::core::option::Option::Some(OptimizeMode::CodeSize),
                3 => ::core::option::Option::Some(OptimizeMode::LiteRuntime),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for OptimizeMode {
        fn default() -> OptimizeMode {
            OptimizeMode::Speed
        }
    }
    impl ::core::convert::From<OptimizeMode> for i32 {
        fn from(value: OptimizeMode) -> i32 {
            value as i32
        }
    }
    impl OptimizeMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OptimizeMode::Speed => "SPEED",
                OptimizeMode::CodeSize => "CODE_SIZE",
                OptimizeMode::LiteRuntime => "LITE_RUNTIME",
            }
        }
    }
}
pub struct MessageOptions {
    /// Set true to use the old proto1 MessageSet wire format for extensions.
    /// This is provided for backwards-compatibility with the MessageSet wire
    /// format.  You should not use this for any other reason:  It's less
    /// efficient, has fewer features, and is more complicated.
    ///
    /// The message must be defined exactly as follows:
    /// message Foo {
    /// option message_set_wire_format = true;
    /// extensions 4 to max;
    /// }
    /// Note that the message cannot have any defined fields; MessageSets only
    /// have extensions.
    ///
    /// All extensions of your type must be singular messages; e.g. they cannot
    /// be int32s, enums, or repeated messages.
    ///
    /// Because this is an option, the above two restrictions are not enforced by
    /// the protocol compiler.
    #[prost(bool, optional, tag = "1", default = "false")]
    pub message_set_wire_format: ::core::option::Option<bool>,
    /// Disables the generation of the standard "descriptor()" accessor, which can
    /// conflict with a field of the same name.  This is meant to make migration
    /// from proto1 easier; new code should avoid fields named "descriptor".
    #[prost(bool, optional, tag = "2", default = "false")]
    pub no_standard_descriptor_accessor: ::core::option::Option<bool>,
    /// Is this message deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the message, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating messages.
    #[prost(bool, optional, tag = "3", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    /// Whether the message is an automatically generated map entry type for the
    /// maps field.
    ///
    /// For maps fields:
    /// map\<KeyType, ValueType> map_field = 1;
    /// The parsed descriptor looks like:
    /// message MapFieldEntry {
    /// option map_entry = true;
    /// optional KeyType key = 1;
    /// optional ValueType value = 2;
    /// }
    /// repeated MapFieldEntry map_field = 1;
    ///
    /// Implementations may choose not to generate the map_entry=true message, but
    /// use a native map in the target language to hold the keys and values.
    /// The reflection APIs in such implementations still need to work as
    /// if the field is a repeated message field.
    ///
    /// NOTE: Do not set the option in .proto files. Always use the maps syntax
    /// instead. The option should only be implicitly set by the proto compiler
    /// parser.
    #[prost(bool, optional, tag = "7")]
    pub map_entry: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for MessageOptions {
    #[inline]
    fn clone(&self) -> MessageOptions {
        MessageOptions {
            message_set_wire_format: ::core::clone::Clone::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: ::core::clone::Clone::clone(
                &self.no_standard_descriptor_accessor,
            ),
            deprecated: ::core::clone::Clone::clone(&self.deprecated),
            map_entry: ::core::clone::Clone::clone(&self.map_entry),
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MessageOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MessageOptions {
    #[inline]
    fn eq(&self, other: &MessageOptions) -> bool {
        self.message_set_wire_format == other.message_set_wire_format
            && self.no_standard_descriptor_accessor == other.no_standard_descriptor_accessor
            && self.deprecated == other.deprecated
            && self.map_entry == other.map_entry
            && self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for MessageOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(1u32, &self.message_set_wire_format, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(2u32, &self.no_standard_descriptor_accessor, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(3u32, &self.deprecated, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(7u32, &self.map_entry, buf);
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "MessageOptions";
        match tag {
            1u32 => {
                let mut value = &mut self.message_set_wire_format;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "message_set_wire_format");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.no_standard_descriptor_accessor;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "no_standard_descriptor_accessor");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.deprecated;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "deprecated");
                    error
                })
            }
            7u32 => {
                let mut value = &mut self.map_entry;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "map_entry");
                    error
                })
            }
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self
            .message_set_wire_format
            .as_ref()
            .map_or(0, |value| ::prost::encoding::bool::encoded_len(1u32, value))
            + self
                .no_standard_descriptor_accessor
                .as_ref()
                .map_or(0, |value| ::prost::encoding::bool::encoded_len(2u32, value))
            + self
                .deprecated
                .as_ref()
                .map_or(0, |value| ::prost::encoding::bool::encoded_len(3u32, value))
            + self
                .map_entry
                .as_ref()
                .map_or(0, |value| ::prost::encoding::bool::encoded_len(7u32, value))
            + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.message_set_wire_format = ::core::option::Option::None;
        self.no_standard_descriptor_accessor = ::core::option::Option::None;
        self.deprecated = ::core::option::Option::None;
        self.map_entry = ::core::option::Option::None;
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for MessageOptions {
    fn default() -> Self {
        MessageOptions {
            message_set_wire_format: ::core::option::Option::None,
            no_standard_descriptor_accessor: ::core::option::Option::None,
            deprecated: ::core::option::Option::None,
            map_entry: ::core::option::Option::None,
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for MessageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("MessageOptions");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.message_set_wire_format)
            };
            builder.field("message_set_wire_format", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.no_standard_descriptor_accessor)
            };
            builder.field("no_standard_descriptor_accessor", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.deprecated)
            };
            builder.field("deprecated", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.map_entry)
            };
            builder.field("map_entry", &wrapper)
        };
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl MessageOptions {
    ///Returns the value of `message_set_wire_format`, or the default value if `message_set_wire_format` is unset.
    pub fn message_set_wire_format(&self) -> bool {
        match self.message_set_wire_format {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `no_standard_descriptor_accessor`, or the default value if `no_standard_descriptor_accessor` is unset.
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        match self.no_standard_descriptor_accessor {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `deprecated`, or the default value if `deprecated` is unset.
    pub fn deprecated(&self) -> bool {
        match self.deprecated {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `map_entry`, or the default value if `map_entry` is unset.
    pub fn map_entry(&self) -> bool {
        match self.map_entry {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
}
pub struct FieldOptions {
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    #[prost(
        enumeration = "field_options::CType",
        optional,
        tag = "1",
        default = "String"
    )]
    pub ctype: ::core::option::Option<i32>,
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    #[prost(bool, optional, tag = "2")]
    pub packed: ::core::option::Option<bool>,
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    #[prost(
        enumeration = "field_options::JsType",
        optional,
        tag = "6",
        default = "JsNormal"
    )]
    pub jstype: ::core::option::Option<i32>,
    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    #[prost(bool, optional, tag = "5", default = "false")]
    pub lazy: ::core::option::Option<bool>,
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    #[prost(bool, optional, tag = "3", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    /// For Google-internal migration only. Do not use.
    #[prost(bool, optional, tag = "10", default = "false")]
    pub weak: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for FieldOptions {
    #[inline]
    fn clone(&self) -> FieldOptions {
        FieldOptions {
            ctype: ::core::clone::Clone::clone(&self.ctype),
            packed: ::core::clone::Clone::clone(&self.packed),
            jstype: ::core::clone::Clone::clone(&self.jstype),
            lazy: ::core::clone::Clone::clone(&self.lazy),
            deprecated: ::core::clone::Clone::clone(&self.deprecated),
            weak: ::core::clone::Clone::clone(&self.weak),
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FieldOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FieldOptions {
    #[inline]
    fn eq(&self, other: &FieldOptions) -> bool {
        self.ctype == other.ctype
            && self.packed == other.packed
            && self.jstype == other.jstype
            && self.lazy == other.lazy
            && self.deprecated == other.deprecated
            && self.weak == other.weak
            && self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for FieldOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufEnum,
            _,
            _,
        >(1u32, &self.ctype, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(2u32, &self.packed, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(3u32, &self.deprecated, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(5u32, &self.lazy, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufEnum,
            _,
            _,
        >(6u32, &self.jstype, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(10u32, &self.weak, buf);
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "FieldOptions";
        match tag {
            1u32 => {
                let mut value = &mut self.ctype;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "ctype");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.packed;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "packed");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.deprecated;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "deprecated");
                    error
                })
            }
            5u32 => {
                let mut value = &mut self.lazy;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "lazy");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.jstype;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "jstype");
                    error
                })
            }
            10u32 => {
                let mut value = &mut self.weak;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "weak");
                    error
                })
            }
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.ctype.as_ref().map_or(0, |value| {
            ::prost::encoding::int32::encoded_len(1u32, value)
        }) + self
            .packed
            .as_ref()
            .map_or(0, |value| ::prost::encoding::bool::encoded_len(2u32, value))
            + self
                .deprecated
                .as_ref()
                .map_or(0, |value| ::prost::encoding::bool::encoded_len(3u32, value))
            + self
                .lazy
                .as_ref()
                .map_or(0, |value| ::prost::encoding::bool::encoded_len(5u32, value))
            + self.jstype.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(6u32, value)
            })
            + self.weak.as_ref().map_or(0, |value| {
                ::prost::encoding::bool::encoded_len(10u32, value)
            })
            + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.ctype = ::core::option::Option::None;
        self.packed = ::core::option::Option::None;
        self.deprecated = ::core::option::Option::None;
        self.lazy = ::core::option::Option::None;
        self.jstype = ::core::option::Option::None;
        self.weak = ::core::option::Option::None;
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for FieldOptions {
    fn default() -> Self {
        FieldOptions {
            ctype: ::core::option::Option::None,
            packed: ::core::option::Option::None,
            deprecated: ::core::option::Option::None,
            lazy: ::core::option::Option::None,
            jstype: ::core::option::Option::None,
            weak: ::core::option::Option::None,
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for FieldOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("FieldOptions");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        struct Inner<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for Inner<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                match field_options::CType::from_i32(*self.0) {
                                    None => ::core::fmt::Debug::fmt(&self.0, f),
                                    Some(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.ctype)
            };
            builder.field("ctype", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.packed)
            };
            builder.field("packed", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        struct Inner<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for Inner<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                match field_options::JsType::from_i32(*self.0) {
                                    None => ::core::fmt::Debug::fmt(&self.0, f),
                                    Some(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.jstype)
            };
            builder.field("jstype", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.lazy)
            };
            builder.field("lazy", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.deprecated)
            };
            builder.field("deprecated", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.weak)
            };
            builder.field("weak", &wrapper)
        };
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl FieldOptions {
    ///Returns the enum value of `ctype`, or the default if the field is unset or set to an invalid enum value.
    pub fn ctype(&self) -> field_options::CType {
        self.ctype
            .and_then(field_options::CType::from_i32)
            .unwrap_or(field_options::CType::String)
    }
    ///Sets `ctype` to the provided enum value.
    pub fn set_ctype(&mut self, value: field_options::CType) {
        self.ctype = ::core::option::Option::Some(value as i32);
    }
    ///Returns the value of `packed`, or the default value if `packed` is unset.
    pub fn packed(&self) -> bool {
        match self.packed {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `deprecated`, or the default value if `deprecated` is unset.
    pub fn deprecated(&self) -> bool {
        match self.deprecated {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `lazy`, or the default value if `lazy` is unset.
    pub fn lazy(&self) -> bool {
        match self.lazy {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the enum value of `jstype`, or the default if the field is unset or set to an invalid enum value.
    pub fn jstype(&self) -> field_options::JsType {
        self.jstype
            .and_then(field_options::JsType::from_i32)
            .unwrap_or(field_options::JsType::JsNormal)
    }
    ///Sets `jstype` to the provided enum value.
    pub fn set_jstype(&mut self, value: field_options::JsType) {
        self.jstype = ::core::option::Option::Some(value as i32);
    }
    ///Returns the value of `weak`, or the default value if `weak` is unset.
    pub fn weak(&self) -> bool {
        match self.weak {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
}
/// Nested message and enum types in `FieldOptions`.
pub mod field_options {
    #[repr(i32)]
    pub enum CType {
        /// Default mode.
        String = 0,
        Cord = 1,
        StringPiece = 2,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CType {
        #[inline]
        fn clone(&self) -> CType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for CType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                CType::String => ::core::fmt::Formatter::write_str(f, "String"),
                CType::Cord => ::core::fmt::Formatter::write_str(f, "Cord"),
                CType::StringPiece => ::core::fmt::Formatter::write_str(f, "StringPiece"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CType {
        #[inline]
        fn eq(&self, other: &CType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for CType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for CType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for CType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for CType {
        #[inline]
        fn partial_cmp(&self, other: &CType) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for CType {
        #[inline]
        fn cmp(&self, other: &CType) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    impl CType {
        ///Returns `true` if `value` is a variant of `CType`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                _ => false,
            }
        }
        ///Converts an `i32` to a `CType`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<CType> {
            match value {
                0 => ::core::option::Option::Some(CType::String),
                1 => ::core::option::Option::Some(CType::Cord),
                2 => ::core::option::Option::Some(CType::StringPiece),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for CType {
        fn default() -> CType {
            CType::String
        }
    }
    impl ::core::convert::From<CType> for i32 {
        fn from(value: CType) -> i32 {
            value as i32
        }
    }
    impl CType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CType::String => "STRING",
                CType::Cord => "CORD",
                CType::StringPiece => "STRING_PIECE",
            }
        }
    }
    #[repr(i32)]
    pub enum JsType {
        /// Use the default type.
        JsNormal = 0,
        /// Use JavaScript strings.
        JsString = 1,
        /// Use JavaScript numbers.
        JsNumber = 2,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for JsType {
        #[inline]
        fn clone(&self) -> JsType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for JsType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for JsType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                JsType::JsNormal => ::core::fmt::Formatter::write_str(f, "JsNormal"),
                JsType::JsString => ::core::fmt::Formatter::write_str(f, "JsString"),
                JsType::JsNumber => ::core::fmt::Formatter::write_str(f, "JsNumber"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for JsType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for JsType {
        #[inline]
        fn eq(&self, other: &JsType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for JsType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for JsType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for JsType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for JsType {
        #[inline]
        fn partial_cmp(&self, other: &JsType) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for JsType {
        #[inline]
        fn cmp(&self, other: &JsType) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    impl JsType {
        ///Returns `true` if `value` is a variant of `JsType`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                _ => false,
            }
        }
        ///Converts an `i32` to a `JsType`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<JsType> {
            match value {
                0 => ::core::option::Option::Some(JsType::JsNormal),
                1 => ::core::option::Option::Some(JsType::JsString),
                2 => ::core::option::Option::Some(JsType::JsNumber),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for JsType {
        fn default() -> JsType {
            JsType::JsNormal
        }
    }
    impl ::core::convert::From<JsType> for i32 {
        fn from(value: JsType) -> i32 {
            value as i32
        }
    }
    impl JsType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JsType::JsNormal => "JS_NORMAL",
                JsType::JsString => "JS_STRING",
                JsType::JsNumber => "JS_NUMBER",
            }
        }
    }
}
pub struct OneofOptions {
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for OneofOptions {
    #[inline]
    fn clone(&self) -> OneofOptions {
        OneofOptions {
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OneofOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OneofOptions {
    #[inline]
    fn eq(&self, other: &OneofOptions) -> bool {
        self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for OneofOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "OneofOptions";
        match tag {
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for OneofOptions {
    fn default() -> Self {
        OneofOptions {
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for OneofOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("OneofOptions");
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
pub struct EnumOptions {
    /// Set this option to true to allow mapping different tag names to the same
    /// value.
    #[prost(bool, optional, tag = "2")]
    pub allow_alias: ::core::option::Option<bool>,
    /// Is this enum deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating enums.
    #[prost(bool, optional, tag = "3", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for EnumOptions {
    #[inline]
    fn clone(&self) -> EnumOptions {
        EnumOptions {
            allow_alias: ::core::clone::Clone::clone(&self.allow_alias),
            deprecated: ::core::clone::Clone::clone(&self.deprecated),
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumOptions {
    #[inline]
    fn eq(&self, other: &EnumOptions) -> bool {
        self.allow_alias == other.allow_alias
            && self.deprecated == other.deprecated
            && self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for EnumOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(2u32, &self.allow_alias, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(3u32, &self.deprecated, buf);
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "EnumOptions";
        match tag {
            2u32 => {
                let mut value = &mut self.allow_alias;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "allow_alias");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.deprecated;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "deprecated");
                    error
                })
            }
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self
            .allow_alias
            .as_ref()
            .map_or(0, |value| ::prost::encoding::bool::encoded_len(2u32, value))
            + self
                .deprecated
                .as_ref()
                .map_or(0, |value| ::prost::encoding::bool::encoded_len(3u32, value))
            + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.allow_alias = ::core::option::Option::None;
        self.deprecated = ::core::option::Option::None;
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for EnumOptions {
    fn default() -> Self {
        EnumOptions {
            allow_alias: ::core::option::Option::None,
            deprecated: ::core::option::Option::None,
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for EnumOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("EnumOptions");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.allow_alias)
            };
            builder.field("allow_alias", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.deprecated)
            };
            builder.field("deprecated", &wrapper)
        };
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl EnumOptions {
    ///Returns the value of `allow_alias`, or the default value if `allow_alias` is unset.
    pub fn allow_alias(&self) -> bool {
        match self.allow_alias {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the value of `deprecated`, or the default value if `deprecated` is unset.
    pub fn deprecated(&self) -> bool {
        match self.deprecated {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
}
pub struct EnumValueOptions {
    /// Is this enum value deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating enum values.
    #[prost(bool, optional, tag = "1", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for EnumValueOptions {
    #[inline]
    fn clone(&self) -> EnumValueOptions {
        EnumValueOptions {
            deprecated: ::core::clone::Clone::clone(&self.deprecated),
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumValueOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumValueOptions {
    #[inline]
    fn eq(&self, other: &EnumValueOptions) -> bool {
        self.deprecated == other.deprecated
            && self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for EnumValueOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(1u32, &self.deprecated, buf);
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "EnumValueOptions";
        match tag {
            1u32 => {
                let mut value = &mut self.deprecated;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "deprecated");
                    error
                })
            }
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self
            .deprecated
            .as_ref()
            .map_or(0, |value| ::prost::encoding::bool::encoded_len(1u32, value))
            + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.deprecated = ::core::option::Option::None;
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for EnumValueOptions {
    fn default() -> Self {
        EnumValueOptions {
            deprecated: ::core::option::Option::None,
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for EnumValueOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("EnumValueOptions");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.deprecated)
            };
            builder.field("deprecated", &wrapper)
        };
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl EnumValueOptions {
    ///Returns the value of `deprecated`, or the default value if `deprecated` is unset.
    pub fn deprecated(&self) -> bool {
        match self.deprecated {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
}
pub struct ServiceOptions {
    /// Is this service deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the service, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating services.
    #[prost(bool, optional, tag = "33", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for ServiceOptions {
    #[inline]
    fn clone(&self) -> ServiceOptions {
        ServiceOptions {
            deprecated: ::core::clone::Clone::clone(&self.deprecated),
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ServiceOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ServiceOptions {
    #[inline]
    fn eq(&self, other: &ServiceOptions) -> bool {
        self.deprecated == other.deprecated
            && self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for ServiceOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(33u32, &self.deprecated, buf);
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "ServiceOptions";
        match tag {
            33u32 => {
                let mut value = &mut self.deprecated;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "deprecated");
                    error
                })
            }
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.deprecated.as_ref().map_or(0, |value| {
            ::prost::encoding::bool::encoded_len(33u32, value)
        }) + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.deprecated = ::core::option::Option::None;
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for ServiceOptions {
    fn default() -> Self {
        ServiceOptions {
            deprecated: ::core::option::Option::None,
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for ServiceOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("ServiceOptions");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.deprecated)
            };
            builder.field("deprecated", &wrapper)
        };
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl ServiceOptions {
    ///Returns the value of `deprecated`, or the default value if `deprecated` is unset.
    pub fn deprecated(&self) -> bool {
        match self.deprecated {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
}
pub struct MethodOptions {
    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    #[prost(bool, optional, tag = "33", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(
        enumeration = "method_options::IdempotencyLevel",
        optional,
        tag = "34",
        default = "IdempotencyUnknown"
    )]
    pub idempotency_level: ::core::option::Option<i32>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[automatically_derived]
impl ::core::clone::Clone for MethodOptions {
    #[inline]
    fn clone(&self) -> MethodOptions {
        MethodOptions {
            deprecated: ::core::clone::Clone::clone(&self.deprecated),
            idempotency_level: ::core::clone::Clone::clone(&self.idempotency_level),
            uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MethodOptions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MethodOptions {
    #[inline]
    fn eq(&self, other: &MethodOptions) -> bool {
        self.deprecated == other.deprecated
            && self.idempotency_level == other.idempotency_level
            && self.uninterpreted_option == other.uninterpreted_option
    }
}
impl ::prost::Message for MethodOptions {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBool,
            _,
            _,
        >(33u32, &self.deprecated, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufEnum,
            _,
            _,
        >(34u32, &self.idempotency_level, buf);
        for msg in &self.uninterpreted_option {
            ::prost::encoding::message::encode(999u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "MethodOptions";
        match tag {
            33u32 => {
                let mut value = &mut self.deprecated;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "deprecated");
                    error
                })
            }
            34u32 => {
                let mut value = &mut self.idempotency_level;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "idempotency_level");
                    error
                })
            }
            999u32 => {
                let mut value = &mut self.uninterpreted_option;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "uninterpreted_option");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.deprecated.as_ref().map_or(0, |value| {
            ::prost::encoding::bool::encoded_len(33u32, value)
        }) + self.idempotency_level.as_ref().map_or(0, |value| {
            ::prost::encoding::int32::encoded_len(34u32, value)
        }) + ::prost::encoding::message::encoded_len_repeated(999u32, &self.uninterpreted_option)
    }
    fn clear(&mut self) {
        self.deprecated = ::core::option::Option::None;
        self.idempotency_level = ::core::option::Option::None;
        self.uninterpreted_option.clear();
    }
}
impl ::core::default::Default for MethodOptions {
    fn default() -> Self {
        MethodOptions {
            deprecated: ::core::option::Option::None,
            idempotency_level: ::core::option::Option::None,
            uninterpreted_option: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for MethodOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("MethodOptions");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<bool>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.deprecated)
            };
            builder.field("deprecated", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        struct Inner<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for Inner<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                match method_options::IdempotencyLevel::from_i32(*self.0) {
                                    None => ::core::fmt::Debug::fmt(&self.0, f),
                                    Some(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.idempotency_level)
            };
            builder.field("idempotency_level", &wrapper)
        };
        let builder = {
            let wrapper = &self.uninterpreted_option;
            builder.field("uninterpreted_option", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl MethodOptions {
    ///Returns the value of `deprecated`, or the default value if `deprecated` is unset.
    pub fn deprecated(&self) -> bool {
        match self.deprecated {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => false,
        }
    }
    ///Returns the enum value of `idempotency_level`, or the default if the field is unset or set to an invalid enum value.
    pub fn idempotency_level(&self) -> method_options::IdempotencyLevel {
        self.idempotency_level
            .and_then(method_options::IdempotencyLevel::from_i32)
            .unwrap_or(method_options::IdempotencyLevel::IdempotencyUnknown)
    }
    ///Sets `idempotency_level` to the provided enum value.
    pub fn set_idempotency_level(&mut self, value: method_options::IdempotencyLevel) {
        self.idempotency_level = ::core::option::Option::Some(value as i32);
    }
}
/// Nested message and enum types in `MethodOptions`.
pub mod method_options {
    /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
    /// or neither? HTTP based RPC implementation may choose GET verb for safe
    /// methods, and PUT verb for idempotent methods instead of the default POST.
    #[repr(i32)]
    pub enum IdempotencyLevel {
        IdempotencyUnknown = 0,
        /// implies idempotent
        NoSideEffects = 1,
        /// idempotent, but may have side effects
        Idempotent = 2,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IdempotencyLevel {
        #[inline]
        fn clone(&self) -> IdempotencyLevel {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for IdempotencyLevel {}
    #[automatically_derived]
    impl ::core::fmt::Debug for IdempotencyLevel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                IdempotencyLevel::IdempotencyUnknown => {
                    ::core::fmt::Formatter::write_str(f, "IdempotencyUnknown")
                }
                IdempotencyLevel::NoSideEffects => {
                    ::core::fmt::Formatter::write_str(f, "NoSideEffects")
                }
                IdempotencyLevel::Idempotent => ::core::fmt::Formatter::write_str(f, "Idempotent"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for IdempotencyLevel {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for IdempotencyLevel {
        #[inline]
        fn eq(&self, other: &IdempotencyLevel) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for IdempotencyLevel {}
    #[automatically_derived]
    impl ::core::cmp::Eq for IdempotencyLevel {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for IdempotencyLevel {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for IdempotencyLevel {
        #[inline]
        fn partial_cmp(
            &self,
            other: &IdempotencyLevel,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for IdempotencyLevel {
        #[inline]
        fn cmp(&self, other: &IdempotencyLevel) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    impl IdempotencyLevel {
        ///Returns `true` if `value` is a variant of `IdempotencyLevel`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                _ => false,
            }
        }
        ///Converts an `i32` to a `IdempotencyLevel`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<IdempotencyLevel> {
            match value {
                0 => ::core::option::Option::Some(IdempotencyLevel::IdempotencyUnknown),
                1 => ::core::option::Option::Some(IdempotencyLevel::NoSideEffects),
                2 => ::core::option::Option::Some(IdempotencyLevel::Idempotent),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for IdempotencyLevel {
        fn default() -> IdempotencyLevel {
            IdempotencyLevel::IdempotencyUnknown
        }
    }
    impl ::core::convert::From<IdempotencyLevel> for i32 {
        fn from(value: IdempotencyLevel) -> i32 {
            value as i32
        }
    }
    impl IdempotencyLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IdempotencyLevel::IdempotencyUnknown => "IDEMPOTENCY_UNKNOWN",
                IdempotencyLevel::NoSideEffects => "NO_SIDE_EFFECTS",
                IdempotencyLevel::Idempotent => "IDEMPOTENT",
            }
        }
    }
}
/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
pub struct UninterpretedOption {
    #[prost(message, repeated, tag = "2")]
    pub name: ::prost::alloc::vec::Vec<uninterpreted_option::NamePart>,
    /// The value of the uninterpreted option, in whatever type the tokenizer
    /// identified it as during parsing. Exactly one of these should be set.
    #[prost(string, optional, tag = "3")]
    pub identifier_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "4")]
    pub positive_int_value: ::core::option::Option<u64>,
    #[prost(int64, optional, tag = "5")]
    pub negative_int_value: ::core::option::Option<i64>,
    #[prost(double, optional, tag = "6")]
    pub double_value: ::core::option::Option<f64>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub string_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "8")]
    pub aggregate_value: ::core::option::Option<::prost::alloc::string::String>,
}
#[automatically_derived]
impl ::core::clone::Clone for UninterpretedOption {
    #[inline]
    fn clone(&self) -> UninterpretedOption {
        UninterpretedOption {
            name: ::core::clone::Clone::clone(&self.name),
            identifier_value: ::core::clone::Clone::clone(&self.identifier_value),
            positive_int_value: ::core::clone::Clone::clone(&self.positive_int_value),
            negative_int_value: ::core::clone::Clone::clone(&self.negative_int_value),
            double_value: ::core::clone::Clone::clone(&self.double_value),
            string_value: ::core::clone::Clone::clone(&self.string_value),
            aggregate_value: ::core::clone::Clone::clone(&self.aggregate_value),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UninterpretedOption {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UninterpretedOption {
    #[inline]
    fn eq(&self, other: &UninterpretedOption) -> bool {
        self.name == other.name
            && self.identifier_value == other.identifier_value
            && self.positive_int_value == other.positive_int_value
            && self.negative_int_value == other.negative_int_value
            && self.double_value == other.double_value
            && self.string_value == other.string_value
            && self.aggregate_value == other.aggregate_value
    }
}
impl ::prost::Message for UninterpretedOption {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        for msg in &self.name {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(3u32, &self.identifier_value, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::Uint64,
            _,
            _,
        >(4u32, &self.positive_int_value, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::Int64,
            _,
            _,
        >(5u32, &self.negative_int_value, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufDouble,
            _,
            _,
        >(6u32, &self.double_value, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufBytes,
            _,
            _,
        >(7u32, &self.string_value, buf);
        <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(8u32, &self.aggregate_value, buf);
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "UninterpretedOption";
        match tag {
            2u32 => {
                let mut value = &mut self.name;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "name");
                        error
                    },
                )
            }
            3u32 => {
                let mut value = &mut self.identifier_value;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "identifier_value");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.positive_int_value;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Uint64,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "positive_int_value");
                    error
                })
            }
            5u32 => {
                let mut value = &mut self.negative_int_value;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int64,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "negative_int_value");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.double_value;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufDouble,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "double_value");
                    error
                })
            }
            7u32 => {
                let mut value = &mut self.string_value;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBytes,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "string_value");
                    error
                })
            }
            8u32 => {
                let mut value = &mut self.aggregate_value;
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "aggregate_value");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::message::encoded_len_repeated(2u32, &self.name)
            + self.identifier_value.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(3u32, value)
            })
            + self.positive_int_value.as_ref().map_or(0, |value| {
                ::prost::encoding::uint64::encoded_len(4u32, value)
            })
            + self.negative_int_value.as_ref().map_or(0, |value| {
                ::prost::encoding::int64::encoded_len(5u32, value)
            })
            + self.double_value.as_ref().map_or(0, |value| {
                ::prost::encoding::double::encoded_len(6u32, value)
            })
            + self.string_value.as_ref().map_or(0, |value| {
                ::prost::encoding::bytes::encoded_len(7u32, value)
            })
            + self.aggregate_value.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(8u32, value)
            })
    }
    fn clear(&mut self) {
        self.name.clear();
        self.identifier_value = ::core::option::Option::None;
        self.positive_int_value = ::core::option::Option::None;
        self.negative_int_value = ::core::option::Option::None;
        self.double_value = ::core::option::Option::None;
        self.string_value = ::core::option::Option::None;
        self.aggregate_value = ::core::option::Option::None;
    }
}
impl ::core::default::Default for UninterpretedOption {
    fn default() -> Self {
        UninterpretedOption {
            name: ::core::default::Default::default(),
            identifier_value: ::core::option::Option::None,
            positive_int_value: ::core::option::Option::None,
            negative_int_value: ::core::option::Option::None,
            double_value: ::core::option::Option::None,
            string_value: ::core::option::Option::None,
            aggregate_value: ::core::option::Option::None,
        }
    }
}
impl ::core::fmt::Debug for UninterpretedOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("UninterpretedOption");
        let builder = {
            let wrapper = &self.name;
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.identifier_value)
            };
            builder.field("identifier_value", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<u64>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.positive_int_value)
            };
            builder.field("positive_int_value", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<i64>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.negative_int_value)
            };
            builder.field("negative_int_value", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<f64>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.double_value)
            };
            builder.field("double_value", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a ::core::option::Option<::prost::alloc::vec::Vec<u8>>);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.string_value)
            };
            builder.field("string_value", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::core::option::Option<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn Inner<T>(v: T) -> T {
                            v
                        }
                        ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                    }
                }
                ScalarWrapper(&self.aggregate_value)
            };
            builder.field("aggregate_value", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl UninterpretedOption {
    ///Returns the value of `identifier_value`, or the default value if `identifier_value` is unset.
    pub fn identifier_value(&self) -> &str {
        match self.identifier_value {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
    ///Returns the value of `positive_int_value`, or the default value if `positive_int_value` is unset.
    pub fn positive_int_value(&self) -> u64 {
        match self.positive_int_value {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => 0u64,
        }
    }
    ///Returns the value of `negative_int_value`, or the default value if `negative_int_value` is unset.
    pub fn negative_int_value(&self) -> i64 {
        match self.negative_int_value {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => 0i64,
        }
    }
    ///Returns the value of `double_value`, or the default value if `double_value` is unset.
    pub fn double_value(&self) -> f64 {
        match self.double_value {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => 0f64,
        }
    }
    ///Returns the value of `string_value`, or the default value if `string_value` is unset.
    pub fn string_value(&self) -> &[u8] {
        match self.string_value {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => b"" as &[u8],
        }
    }
    ///Returns the value of `aggregate_value`, or the default value if `aggregate_value` is unset.
    pub fn aggregate_value(&self) -> &str {
        match self.aggregate_value {
            ::core::option::Option::Some(ref val) => &val[..],
            ::core::option::Option::None => "",
        }
    }
}
/// Nested message and enum types in `UninterpretedOption`.
pub mod uninterpreted_option {
    /// The name of the uninterpreted option.  Each string represents a segment in
    /// a dot-separated name.  is_extension is true iff a segment represents an
    /// extension (denoted with parentheses in options specs in .proto files).
    /// E.g.,{ \["foo", false\], \["bar.baz", true\], \["qux", false\] } represents
    /// "foo.(bar.baz).qux".
    pub struct NamePart {
        #[prost(string, required, tag = "1")]
        pub name_part: ::prost::alloc::string::String,
        #[prost(bool, required, tag = "2")]
        pub is_extension: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NamePart {
        #[inline]
        fn clone(&self) -> NamePart {
            NamePart {
                name_part: ::core::clone::Clone::clone(&self.name_part),
                is_extension: ::core::clone::Clone::clone(&self.is_extension),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for NamePart {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for NamePart {
        #[inline]
        fn eq(&self, other: &NamePart) -> bool {
            self.name_part == other.name_part && self.is_extension == other.is_extension
        }
    }
    impl ::prost::Message for NamePart {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Required as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.name_part, buf);
            <::prost::encoding_traits::Required as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufBool,
                _,
                _,
            >(2u32, &self.is_extension, buf);
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "NamePart";
            match tag {
                1u32 => {
                    let mut value = &mut self.name_part;
                    <::prost::encoding_traits::Required as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "name_part");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.is_extension;
                    <::prost::encoding_traits::Required as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufBool,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "is_extension");
                        error
                    })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::string::encoded_len(1u32, &self.name_part)
                + ::prost::encoding::bool::encoded_len(2u32, &self.is_extension)
        }
        fn clear(&mut self) {
            self.name_part.clear();
            self.is_extension = false;
        }
    }
    impl ::core::default::Default for NamePart {
        fn default() -> Self {
            NamePart {
                name_part: ::prost::alloc::string::String::new(),
                is_extension: false,
            }
        }
    }
    impl ::core::fmt::Debug for NamePart {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("NamePart");
            let builder = {
                let wrapper = {
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.name_part)
                };
                builder.field("name_part", &wrapper)
            };
            let builder = {
                let wrapper = {
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.is_extension)
                };
                builder.field("is_extension", &wrapper)
            };
            builder.finish()
        }
    }
}
/// Encapsulates information about the original source file from which a
/// FileDescriptorProto was generated.
pub struct SourceCodeInfo {
    /// A Location identifies a piece of source code in a .proto file which
    /// corresponds to a particular definition.  This information is intended
    /// to be useful to IDEs, code indexers, documentation generators, and similar
    /// tools.
    ///
    /// For example, say we have a file like:
    /// message Foo {
    /// optional string foo = 1;
    /// }
    /// Let's look at just the field definition:
    /// optional string foo = 1;
    /// ^       ^^     ^^  ^  ^^^
    /// a       bc     de  f  ghi
    /// We have the following locations:
    /// span   path               represents
    /// \[a,i)  \[ 4, 0, 2, 0 \]     The whole field definition.
    /// \[a,b)  \[ 4, 0, 2, 0, 4 \]  The label (optional).
    /// \[c,d)  \[ 4, 0, 2, 0, 5 \]  The type (string).
    /// \[e,f)  \[ 4, 0, 2, 0, 1 \]  The name (foo).
    /// \[g,h)  \[ 4, 0, 2, 0, 3 \]  The number (1).
    ///
    /// Notes:
    ///
    /// * A location may refer to a repeated field itself (i.e. not to any
    ///   particular index within it).  This is used whenever a set of elements are
    ///   logically enclosed in a single code segment.  For example, an entire
    ///   extend block (possibly containing multiple extension definitions) will
    ///   have an outer location whose path refers to the "extensions" repeated
    ///   field without an index.
    /// * Multiple locations may have the same path.  This happens when a single
    ///   logical declaration is spread out across multiple places.  The most
    ///   obvious example is the "extend" block again -- there may be multiple
    ///   extend blocks in the same scope, each of which will have the same path.
    /// * A location's span is not always a subset of its parent's span.  For
    ///   example, the "extendee" of an extension declaration appears at the
    ///   beginning of the "extend" block and is shared by all extensions within
    ///   the block.
    /// * Just because a location's span is a subset of some other location's span
    ///   does not mean that it is a descendant.  For example, a "group" defines
    ///   both a type and a field in a single declaration.  Thus, the locations
    ///   corresponding to the type and field and their components will overlap.
    /// * Code which tries to interpret locations should probably be designed to
    ///   ignore those that it doesn't understand, as more types of locations could
    ///   be recorded in the future.
    #[prost(message, repeated, tag = "1")]
    pub location: ::prost::alloc::vec::Vec<source_code_info::Location>,
}
#[automatically_derived]
impl ::core::clone::Clone for SourceCodeInfo {
    #[inline]
    fn clone(&self) -> SourceCodeInfo {
        SourceCodeInfo {
            location: ::core::clone::Clone::clone(&self.location),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SourceCodeInfo {}
#[automatically_derived]
impl ::core::cmp::PartialEq for SourceCodeInfo {
    #[inline]
    fn eq(&self, other: &SourceCodeInfo) -> bool {
        self.location == other.location
    }
}
impl ::prost::Message for SourceCodeInfo {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        for msg in &self.location {
            ::prost::encoding::message::encode(1u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "SourceCodeInfo";
        match tag {
            1u32 => {
                let mut value = &mut self.location;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "location");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.location)
    }
    fn clear(&mut self) {
        self.location.clear();
    }
}
impl ::core::default::Default for SourceCodeInfo {
    fn default() -> Self {
        SourceCodeInfo {
            location: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for SourceCodeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("SourceCodeInfo");
        let builder = {
            let wrapper = &self.location;
            builder.field("location", &wrapper)
        };
        builder.finish()
    }
}
/// Nested message and enum types in `SourceCodeInfo`.
pub mod source_code_info {
    pub struct Location {
        /// Identifies which part of the FileDescriptorProto was defined at this
        /// location.
        ///
        /// Each element is a field number or an index.  They form a path from
        /// the root FileDescriptorProto to the place where the definition.  For
        /// example, this path:
        /// \[ 4, 3, 2, 7, 1 \]
        /// refers to:
        /// file.message_type(3)  // 4, 3
        /// .field(7)         // 2, 7
        /// .name()           // 1
        /// This is because FileDescriptorProto.message_type has field number 4:
        /// repeated DescriptorProto message_type = 4;
        /// and DescriptorProto.field has field number 2:
        /// repeated FieldDescriptorProto field = 2;
        /// and FieldDescriptorProto.name has field number 1:
        /// optional string name = 1;
        ///
        /// Thus, the above path gives the location of a field name.  If we removed
        /// the last element:
        /// \[ 4, 3, 2, 7 \]
        /// this path refers to the whole field declaration (from the beginning
        /// of the label to the terminating semicolon).
        #[prost(int32, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<i32>,
        /// Always has exactly three or four elements: start line, start column,
        /// end line (optional, otherwise assumed same as start line), end column.
        /// These are packed into a single field for efficiency.  Note that line
        /// and column numbers are zero-based -- typically you will want to add
        /// 1 to each before displaying to a user.
        #[prost(int32, repeated, tag = "2")]
        pub span: ::prost::alloc::vec::Vec<i32>,
        /// If this SourceCodeInfo represents a complete declaration, these are any
        /// comments appearing before and after the declaration which appear to be
        /// attached to the declaration.
        ///
        /// A series of line comments appearing on consecutive lines, with no other
        /// tokens appearing on those lines, will be treated as a single comment.
        ///
        /// leading_detached_comments will keep paragraphs of comments that appear
        /// before (but not connected to) the current element. Each paragraph,
        /// separated by empty lines, will be one comment element in the repeated
        /// field.
        ///
        /// Only the comment content is provided; comment markers (e.g. //) are
        /// stripped out.  For block comments, leading whitespace and an asterisk
        /// will be stripped from the beginning of each line other than the first.
        /// Newlines are included in the output.
        ///
        /// Examples:
        ///
        /// optional int32 foo = 1;  // Comment attached to foo.
        /// // Comment attached to bar.
        /// optional int32 bar = 2;
        ///
        /// optional string baz = 3;
        /// // Comment attached to baz.
        /// // Another line attached to baz.
        ///
        /// // Comment attached to qux.
        /// //
        /// // Another line attached to qux.
        /// optional double qux = 4;
        ///
        /// // Detached comment for corge. This is not leading or trailing comments
        /// // to qux or corge because there are blank lines separating it from
        /// // both.
        ///
        /// // Detached comment for corge paragraph 2.
        ///
        /// optional string corge = 5;
        /// /\* Block comment attached
        /// \* to corge.  Leading asterisks
        /// \* will be removed. */
        /// /* Block comment attached to
        /// \* grault. \*/
        /// optional int32 grault = 6;
        ///
        /// // ignored detached comments.
        #[prost(string, optional, tag = "3")]
        pub leading_comments: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "4")]
        pub trailing_comments: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "6")]
        pub leading_detached_comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Location {
        #[inline]
        fn clone(&self) -> Location {
            Location {
                path: ::core::clone::Clone::clone(&self.path),
                span: ::core::clone::Clone::clone(&self.span),
                leading_comments: ::core::clone::Clone::clone(&self.leading_comments),
                trailing_comments: ::core::clone::Clone::clone(&self.trailing_comments),
                leading_detached_comments: ::core::clone::Clone::clone(
                    &self.leading_detached_comments,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Location {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Location {
        #[inline]
        fn eq(&self, other: &Location) -> bool {
            self.path == other.path
                && self.span == other.span
                && self.leading_comments == other.leading_comments
                && self.trailing_comments == other.trailing_comments
                && self.leading_detached_comments == other.leading_detached_comments
        }
    }
    impl ::prost::Message for Location {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Packed as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(1u32, &self.path, buf);
            <::prost::encoding_traits::Packed as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(2u32, &self.span, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(3u32, &self.leading_comments, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(4u32, &self.trailing_comments, buf);
            <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(6u32, &self.leading_detached_comments, buf);
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "Location";
            match tag {
                1u32 => {
                    let mut value = &mut self.path;
                    <::prost::encoding_traits::Packed as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "path");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.span;
                    <::prost::encoding_traits::Packed as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "span");
                        error
                    })
                }
                3u32 => {
                    let mut value = &mut self.leading_comments;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "leading_comments");
                        error
                    })
                }
                4u32 => {
                    let mut value = &mut self.trailing_comments;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "trailing_comments");
                        error
                    })
                }
                6u32 => {
                    let mut value = &mut self.leading_detached_comments;
                    <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "leading_detached_comments");
                        error
                    })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::int32::encoded_len_packed(1u32, &self.path)
                + ::prost::encoding::int32::encoded_len_packed(2u32, &self.span)
                + self.leading_comments.as_ref().map_or(0, |value| {
                    ::prost::encoding::string::encoded_len(3u32, value)
                })
                + self.trailing_comments.as_ref().map_or(0, |value| {
                    ::prost::encoding::string::encoded_len(4u32, value)
                })
                + ::prost::encoding::string::encoded_len_repeated(
                    6u32,
                    &self.leading_detached_comments,
                )
        }
        fn clear(&mut self) {
            self.path.clear();
            self.span.clear();
            self.leading_comments = ::core::option::Option::None;
            self.trailing_comments = ::core::option::Option::None;
            self.leading_detached_comments.clear();
        }
    }
    impl ::core::default::Default for Location {
        fn default() -> Self {
            Location {
                path: ::prost::alloc::vec::Vec::new(),
                span: ::prost::alloc::vec::Vec::new(),
                leading_comments: ::core::option::Option::None,
                trailing_comments: ::core::option::Option::None,
                leading_detached_comments: ::prost::alloc::vec::Vec::new(),
            }
        }
    }
    impl ::core::fmt::Debug for Location {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Location");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.path)
                };
                builder.field("path", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.span)
                };
                builder.field("span", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.leading_comments)
                };
                builder.field("leading_comments", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.trailing_comments)
                };
                builder.field("trailing_comments", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.leading_detached_comments)
                };
                builder.field("leading_detached_comments", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl Location {
        ///Returns the value of `leading_comments`, or the default value if `leading_comments` is unset.
        pub fn leading_comments(&self) -> &str {
            match self.leading_comments {
                ::core::option::Option::Some(ref val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
        ///Returns the value of `trailing_comments`, or the default value if `trailing_comments` is unset.
        pub fn trailing_comments(&self) -> &str {
            match self.trailing_comments {
                ::core::option::Option::Some(ref val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
    }
}
/// Describes the relationship between generated code and its original source
/// file. A GeneratedCodeInfo message is associated with only one generated
/// source file, but may contain references to different source .proto files.
pub struct GeneratedCodeInfo {
    /// An Annotation connects some span of text in generated code to an element
    /// of its generating .proto file.
    #[prost(message, repeated, tag = "1")]
    pub annotation: ::prost::alloc::vec::Vec<generated_code_info::Annotation>,
}
#[automatically_derived]
impl ::core::clone::Clone for GeneratedCodeInfo {
    #[inline]
    fn clone(&self) -> GeneratedCodeInfo {
        GeneratedCodeInfo {
            annotation: ::core::clone::Clone::clone(&self.annotation),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for GeneratedCodeInfo {}
#[automatically_derived]
impl ::core::cmp::PartialEq for GeneratedCodeInfo {
    #[inline]
    fn eq(&self, other: &GeneratedCodeInfo) -> bool {
        self.annotation == other.annotation
    }
}
impl ::prost::Message for GeneratedCodeInfo {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        for msg in &self.annotation {
            ::prost::encoding::message::encode(1u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "GeneratedCodeInfo";
        match tag {
            1u32 => {
                let mut value = &mut self.annotation;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "annotation");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.annotation)
    }
    fn clear(&mut self) {
        self.annotation.clear();
    }
}
impl ::core::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        GeneratedCodeInfo {
            annotation: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for GeneratedCodeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("GeneratedCodeInfo");
        let builder = {
            let wrapper = &self.annotation;
            builder.field("annotation", &wrapper)
        };
        builder.finish()
    }
}
/// Nested message and enum types in `GeneratedCodeInfo`.
pub mod generated_code_info {
    pub struct Annotation {
        /// Identifies the element in the original source .proto file. This field
        /// is formatted the same as SourceCodeInfo.Location.path.
        #[prost(int32, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<i32>,
        /// Identifies the filesystem path to the original source .proto.
        #[prost(string, optional, tag = "2")]
        pub source_file: ::core::option::Option<::prost::alloc::string::String>,
        /// Identifies the starting offset in bytes in the generated code
        /// that relates to the identified object.
        #[prost(int32, optional, tag = "3")]
        pub begin: ::core::option::Option<i32>,
        /// Identifies the ending offset in bytes in the generated code that
        /// relates to the identified offset. The end offset should be one past
        /// the last relevant byte (so the length of the text = end - begin).
        #[prost(int32, optional, tag = "4")]
        pub end: ::core::option::Option<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Annotation {
        #[inline]
        fn clone(&self) -> Annotation {
            Annotation {
                path: ::core::clone::Clone::clone(&self.path),
                source_file: ::core::clone::Clone::clone(&self.source_file),
                begin: ::core::clone::Clone::clone(&self.begin),
                end: ::core::clone::Clone::clone(&self.end),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Annotation {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Annotation {
        #[inline]
        fn eq(&self, other: &Annotation) -> bool {
            self.path == other.path
                && self.source_file == other.source_file
                && self.begin == other.begin
                && self.end == other.end
        }
    }
    impl ::prost::Message for Annotation {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Packed as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(1u32, &self.path, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(2u32, &self.source_file, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(3u32, &self.begin, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(4u32, &self.end, buf);
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "Annotation";
            match tag {
                1u32 => {
                    let mut value = &mut self.path;
                    <::prost::encoding_traits::Packed as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "path");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.source_file;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "source_file");
                        error
                    })
                }
                3u32 => {
                    let mut value = &mut self.begin;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "begin");
                        error
                    })
                }
                4u32 => {
                    let mut value = &mut self.end;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "end");
                        error
                    })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::int32::encoded_len_packed(1u32, &self.path)
                + self.source_file.as_ref().map_or(0, |value| {
                    ::prost::encoding::string::encoded_len(2u32, value)
                })
                + self.begin.as_ref().map_or(0, |value| {
                    ::prost::encoding::int32::encoded_len(3u32, value)
                })
                + self.end.as_ref().map_or(0, |value| {
                    ::prost::encoding::int32::encoded_len(4u32, value)
                })
        }
        fn clear(&mut self) {
            self.path.clear();
            self.source_file = ::core::option::Option::None;
            self.begin = ::core::option::Option::None;
            self.end = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for Annotation {
        fn default() -> Self {
            Annotation {
                path: ::prost::alloc::vec::Vec::new(),
                source_file: ::core::option::Option::None,
                begin: ::core::option::Option::None,
                end: ::core::option::Option::None,
            }
        }
    }
    impl ::core::fmt::Debug for Annotation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Annotation");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.path)
                };
                builder.field("path", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.source_file)
                };
                builder.field("source_file", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.begin)
                };
                builder.field("begin", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.end)
                };
                builder.field("end", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl Annotation {
        ///Returns the value of `source_file`, or the default value if `source_file` is unset.
        pub fn source_file(&self) -> &str {
            match self.source_file {
                ::core::option::Option::Some(ref val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
        ///Returns the value of `begin`, or the default value if `begin` is unset.
        pub fn begin(&self) -> i32 {
            match self.begin {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `end`, or the default value if `end` is unset.
        pub fn end(&self) -> i32 {
            match self.end {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
    }
}
/// `Any` contains an arbitrary serialized protocol buffer message along with a
/// URL that describes the type of the serialized message.
///
/// Protobuf library provides support to pack/unpack Any values in the form
/// of utility functions or additional generated methods of the Any type.
///
/// Example 1: Pack and unpack a message in C++.
///
/// ```text
/// Foo foo = ...;
/// Any any;
/// any.PackFrom(foo);
/// ...
/// if (any.UnpackTo(&foo)) {
///    ...
/// }
/// ```
///
/// Example 2: Pack and unpack a message in Java.
///
/// ```text
/// Foo foo = ...;
/// Any any = Any.pack(foo);
/// ...
/// if (any.is(Foo.class)) {
///    foo = any.unpack(Foo.class);
/// }
/// ```
///
/// Example 3: Pack and unpack a message in Python.
///
/// ```text
/// foo = Foo(...)
/// any = Any()
/// any.Pack(foo)
/// ...
/// if any.Is(Foo.DESCRIPTOR):
///    any.Unpack(foo)
///    ...
/// ```
///
/// Example 4: Pack and unpack a message in Go
///
/// ```text
///   foo := &pb.Foo{...}
///   any, err := anypb.New(foo)
///   if err != nil {
///     ...
///   }
///   ...
///   foo := &pb.Foo{}
///   if err := any.UnmarshalTo(foo); err != nil {
///     ...
///   }
/// ```
///
/// The pack methods provided by protobuf library will by default use
/// 'type.googleapis.com/full.type.name' as the type URL and the unpack
/// methods only use the fully qualified type name after the last '/'
/// in the type URL, for example "foo.bar.com/x/y.z" will yield type
/// name "y.z".
///
/// # JSON
///
/// The JSON representation of an `Any` value uses the regular
/// representation of the deserialized, embedded message, with an
/// additional field `@type` which contains the type URL. Example:
///
/// ```text
/// package google.profile;
/// message Person {
///    string first_name = 1;
///    string last_name = 2;
/// }
///
/// {
///    "@type": "type.googleapis.com/google.profile.Person",
///    "firstName": <string>,
///    "lastName": <string>
/// }
/// ```
///
/// If the embedded message type is well-known and has a custom JSON
/// representation, that representation will be embedded adding a field
/// `value` which holds the custom JSON in addition to the `@type`
/// field. Example (for message \\[google.protobuf.Duration\]\[\\]):
///
/// ```text
/// {
///    "@type": "type.googleapis.com/google.protobuf.Duration",
///    "value": "1.212s"
/// }
/// ```
pub struct Any {
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. This string must contain at least
    /// one "/" character. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    ///
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    ///
    /// * If no scheme is provided, `https` is assumed.
    /// * An HTTP GET on the URL must yield a \\[google.protobuf.Type\]\[\\]
    ///   value in binary format, or produce an error.
    /// * Applications are allowed to cache lookup results based on the
    ///   URL, or have them precompiled into a binary to avoid any
    ///   lookup. Therefore, binary compatibility needs to be preserved
    ///   on changes to types. (Use versioned type names to manage
    ///   breaking changes.)
    ///
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com.
    ///
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    #[prost(string, tag = "1")]
    pub type_url: ::prost::alloc::string::String,
    /// Must be a valid serialized protocol buffer of the above specified type.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[automatically_derived]
impl ::core::clone::Clone for Any {
    #[inline]
    fn clone(&self) -> Any {
        Any {
            type_url: ::core::clone::Clone::clone(&self.type_url),
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Any {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Any {
    #[inline]
    fn eq(&self, other: &Any) -> bool {
        self.type_url == other.type_url && self.value == other.value
    }
}
impl ::prost::Message for Any {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.type_url != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.type_url, buf);
        }
        if self.value != b"" as &[u8] {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufBytes,
                _,
                _,
            >(2u32, &self.value, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Any";
        match tag {
            1u32 => {
                let mut value = &mut self.type_url;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "type_url");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.value;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBytes,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "value");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.type_url != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.type_url)
        } else {
            0
        } + if self.value != b"" as &[u8] {
            ::prost::encoding::bytes::encoded_len(2u32, &self.value)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.type_url.clear();
        self.value.clear();
    }
}
impl ::core::default::Default for Any {
    fn default() -> Self {
        Any {
            type_url: ::prost::alloc::string::String::new(),
            value: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for Any {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Any");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.type_url)
            };
            builder.field("type_url", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.value)
            };
            builder.field("value", &wrapper)
        };
        builder.finish()
    }
}
/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
pub struct SourceContext {
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
}
#[automatically_derived]
impl ::core::clone::Clone for SourceContext {
    #[inline]
    fn clone(&self) -> SourceContext {
        SourceContext {
            file_name: ::core::clone::Clone::clone(&self.file_name),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SourceContext {}
#[automatically_derived]
impl ::core::cmp::PartialEq for SourceContext {
    #[inline]
    fn eq(&self, other: &SourceContext) -> bool {
        self.file_name == other.file_name
    }
}
impl ::prost::Message for SourceContext {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.file_name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.file_name, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "SourceContext";
        match tag {
            1u32 => {
                let mut value = &mut self.file_name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "file_name");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.file_name != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.file_name)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.file_name.clear();
    }
}
impl ::core::default::Default for SourceContext {
    fn default() -> Self {
        SourceContext {
            file_name: ::prost::alloc::string::String::new(),
        }
    }
}
impl ::core::fmt::Debug for SourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("SourceContext");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.file_name)
            };
            builder.field("file_name", &wrapper)
        };
        builder.finish()
    }
}
/// A protocol buffer message type.
pub struct Type {
    /// The fully qualified message name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of fields.
    #[prost(message, repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    /// The list of types appearing in `oneof` definitions in this type.
    #[prost(string, repeated, tag = "3")]
    pub oneofs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The protocol buffer options.
    #[prost(message, repeated, tag = "4")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// The source context.
    #[prost(message, optional, tag = "5")]
    pub source_context: ::core::option::Option<SourceContext>,
    /// The source syntax.
    #[prost(enumeration = "Syntax", tag = "6")]
    pub syntax: i32,
}
#[automatically_derived]
impl ::core::clone::Clone for Type {
    #[inline]
    fn clone(&self) -> Type {
        Type {
            name: ::core::clone::Clone::clone(&self.name),
            fields: ::core::clone::Clone::clone(&self.fields),
            oneofs: ::core::clone::Clone::clone(&self.oneofs),
            options: ::core::clone::Clone::clone(&self.options),
            source_context: ::core::clone::Clone::clone(&self.source_context),
            syntax: ::core::clone::Clone::clone(&self.syntax),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Type {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Type {
    #[inline]
    fn eq(&self, other: &Type) -> bool {
        self.name == other.name
            && self.fields == other.fields
            && self.oneofs == other.oneofs
            && self.options == other.options
            && self.source_context == other.source_context
            && self.syntax == other.syntax
    }
}
impl ::prost::Message for Type {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.name, buf);
        }
        for msg in &self.fields {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
        <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(3u32, &self.oneofs, buf);
        for msg in &self.options {
            ::prost::encoding::message::encode(4u32, msg, buf);
        }
        if let Some(ref msg) = self.source_context {
            ::prost::encoding::message::encode(5u32, msg, buf);
        }
        if self.syntax != Syntax::default() as i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufEnum,
                _,
                _,
            >(6u32, &self.syntax, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Type";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.fields;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "fields");
                        error
                    },
                )
            }
            3u32 => {
                let mut value = &mut self.oneofs;
                <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "oneofs");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "options");
                        error
                    },
                )
            }
            5u32 => {
                let mut value = &mut self.source_context;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "source_context");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.syntax;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "syntax");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.name != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.name)
        } else {
            0
        } + ::prost::encoding::message::encoded_len_repeated(2u32, &self.fields)
            + ::prost::encoding::string::encoded_len_repeated(3u32, &self.oneofs)
            + ::prost::encoding::message::encoded_len_repeated(4u32, &self.options)
            + self
                .source_context
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(5u32, msg))
            + if self.syntax != Syntax::default() as i32 {
                ::prost::encoding::int32::encoded_len(6u32, &self.syntax)
            } else {
                0
            }
    }
    fn clear(&mut self) {
        self.name.clear();
        self.fields.clear();
        self.oneofs.clear();
        self.options.clear();
        self.source_context = ::core::option::Option::None;
        self.syntax = Syntax::default() as i32;
    }
}
impl ::core::default::Default for Type {
    fn default() -> Self {
        Type {
            name: ::prost::alloc::string::String::new(),
            fields: ::core::default::Default::default(),
            oneofs: ::prost::alloc::vec::Vec::new(),
            options: ::core::default::Default::default(),
            source_context: ::core::default::Default::default(),
            syntax: Syntax::default() as i32,
        }
    }
}
impl ::core::fmt::Debug for Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Type");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = &self.fields;
            builder.field("fields", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let mut vec_builder = f.debug_list();
                        for v in self.0 {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            vec_builder.entry(&Inner(v));
                        }
                        vec_builder.finish()
                    }
                }
                ScalarWrapper(&self.oneofs)
            };
            builder.field("oneofs", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = &self.source_context;
            builder.field("source_context", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a i32);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match Syntax::from_i32(*self.0) {
                            None => ::core::fmt::Debug::fmt(&self.0, f),
                            Some(en) => ::core::fmt::Debug::fmt(&en, f),
                        }
                    }
                }
                ScalarWrapper(&self.syntax)
            };
            builder.field("syntax", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl Type {
    ///Returns the enum value of `syntax`, or the default if the field is set to an invalid enum value.
    pub fn syntax(&self) -> Syntax {
        Syntax::from_i32(self.syntax).unwrap_or(Syntax::default())
    }
    ///Sets `syntax` to the provided enum value.
    pub fn set_syntax(&mut self, value: Syntax) {
        self.syntax = value as i32;
    }
}
/// A single field of a message type.
pub struct Field {
    /// The field type.
    #[prost(enumeration = "field::Kind", tag = "1")]
    pub kind: i32,
    /// The field cardinality.
    #[prost(enumeration = "field::Cardinality", tag = "2")]
    pub cardinality: i32,
    /// The field number.
    #[prost(int32, tag = "3")]
    pub number: i32,
    /// The field name.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// The field type URL, without the scheme, for message or enumeration
    /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
    #[prost(string, tag = "6")]
    pub type_url: ::prost::alloc::string::String,
    /// The index of the field type in `Type.oneofs`, for message or enumeration
    /// types. The first type has index 1; zero means the type is not in the list.
    #[prost(int32, tag = "7")]
    pub oneof_index: i32,
    /// Whether to use alternative packed wire representation.
    #[prost(bool, tag = "8")]
    pub packed: bool,
    /// The protocol buffer options.
    #[prost(message, repeated, tag = "9")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// The field JSON name.
    #[prost(string, tag = "10")]
    pub json_name: ::prost::alloc::string::String,
    /// The string value of the default value of this field. Proto2 syntax only.
    #[prost(string, tag = "11")]
    pub default_value: ::prost::alloc::string::String,
}
#[automatically_derived]
impl ::core::clone::Clone for Field {
    #[inline]
    fn clone(&self) -> Field {
        Field {
            kind: ::core::clone::Clone::clone(&self.kind),
            cardinality: ::core::clone::Clone::clone(&self.cardinality),
            number: ::core::clone::Clone::clone(&self.number),
            name: ::core::clone::Clone::clone(&self.name),
            type_url: ::core::clone::Clone::clone(&self.type_url),
            oneof_index: ::core::clone::Clone::clone(&self.oneof_index),
            packed: ::core::clone::Clone::clone(&self.packed),
            options: ::core::clone::Clone::clone(&self.options),
            json_name: ::core::clone::Clone::clone(&self.json_name),
            default_value: ::core::clone::Clone::clone(&self.default_value),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Field {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Field {
    #[inline]
    fn eq(&self, other: &Field) -> bool {
        self.kind == other.kind
            && self.cardinality == other.cardinality
            && self.number == other.number
            && self.name == other.name
            && self.type_url == other.type_url
            && self.oneof_index == other.oneof_index
            && self.packed == other.packed
            && self.options == other.options
            && self.json_name == other.json_name
            && self.default_value == other.default_value
    }
}
impl ::prost::Message for Field {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.kind != field::Kind::default() as i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufEnum,
                _,
                _,
            >(1u32, &self.kind, buf);
        }
        if self.cardinality != field::Cardinality::default() as i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufEnum,
                _,
                _,
            >(2u32, &self.cardinality, buf);
        }
        if self.number != 0i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(3u32, &self.number, buf);
        }
        if self.name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(4u32, &self.name, buf);
        }
        if self.type_url != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(6u32, &self.type_url, buf);
        }
        if self.oneof_index != 0i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(7u32, &self.oneof_index, buf);
        }
        if self.packed != false {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufBool,
                _,
                _,
            >(8u32, &self.packed, buf);
        }
        for msg in &self.options {
            ::prost::encoding::message::encode(9u32, msg, buf);
        }
        if self.json_name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(10u32, &self.json_name, buf);
        }
        if self.default_value != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(11u32, &self.default_value, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Field";
        match tag {
            1u32 => {
                let mut value = &mut self.kind;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "kind");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.cardinality;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "cardinality");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.number;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "number");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.type_url;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "type_url");
                    error
                })
            }
            7u32 => {
                let mut value = &mut self.oneof_index;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "oneof_index");
                    error
                })
            }
            8u32 => {
                let mut value = &mut self.packed;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "packed");
                    error
                })
            }
            9u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "options");
                        error
                    },
                )
            }
            10u32 => {
                let mut value = &mut self.json_name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "json_name");
                    error
                })
            }
            11u32 => {
                let mut value = &mut self.default_value;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "default_value");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.kind != field::Kind::default() as i32 {
            ::prost::encoding::int32::encoded_len(1u32, &self.kind)
        } else {
            0
        } + if self.cardinality != field::Cardinality::default() as i32 {
            ::prost::encoding::int32::encoded_len(2u32, &self.cardinality)
        } else {
            0
        } + if self.number != 0i32 {
            ::prost::encoding::int32::encoded_len(3u32, &self.number)
        } else {
            0
        } + if self.name != "" {
            ::prost::encoding::string::encoded_len(4u32, &self.name)
        } else {
            0
        } + if self.type_url != "" {
            ::prost::encoding::string::encoded_len(6u32, &self.type_url)
        } else {
            0
        } + if self.oneof_index != 0i32 {
            ::prost::encoding::int32::encoded_len(7u32, &self.oneof_index)
        } else {
            0
        } + if self.packed != false {
            ::prost::encoding::bool::encoded_len(8u32, &self.packed)
        } else {
            0
        } + ::prost::encoding::message::encoded_len_repeated(9u32, &self.options)
            + if self.json_name != "" {
                ::prost::encoding::string::encoded_len(10u32, &self.json_name)
            } else {
                0
            }
            + if self.default_value != "" {
                ::prost::encoding::string::encoded_len(11u32, &self.default_value)
            } else {
                0
            }
    }
    fn clear(&mut self) {
        self.kind = field::Kind::default() as i32;
        self.cardinality = field::Cardinality::default() as i32;
        self.number = 0i32;
        self.name.clear();
        self.type_url.clear();
        self.oneof_index = 0i32;
        self.packed = false;
        self.options.clear();
        self.json_name.clear();
        self.default_value.clear();
    }
}
impl ::core::default::Default for Field {
    fn default() -> Self {
        Field {
            kind: field::Kind::default() as i32,
            cardinality: field::Cardinality::default() as i32,
            number: 0i32,
            name: ::prost::alloc::string::String::new(),
            type_url: ::prost::alloc::string::String::new(),
            oneof_index: 0i32,
            packed: false,
            options: ::core::default::Default::default(),
            json_name: ::prost::alloc::string::String::new(),
            default_value: ::prost::alloc::string::String::new(),
        }
    }
}
impl ::core::fmt::Debug for Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Field");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a i32);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match field::Kind::from_i32(*self.0) {
                            None => ::core::fmt::Debug::fmt(&self.0, f),
                            Some(en) => ::core::fmt::Debug::fmt(&en, f),
                        }
                    }
                }
                ScalarWrapper(&self.kind)
            };
            builder.field("kind", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a i32);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match field::Cardinality::from_i32(*self.0) {
                            None => ::core::fmt::Debug::fmt(&self.0, f),
                            Some(en) => ::core::fmt::Debug::fmt(&en, f),
                        }
                    }
                }
                ScalarWrapper(&self.cardinality)
            };
            builder.field("cardinality", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.number)
            };
            builder.field("number", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.type_url)
            };
            builder.field("type_url", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.oneof_index)
            };
            builder.field("oneof_index", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.packed)
            };
            builder.field("packed", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.json_name)
            };
            builder.field("json_name", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.default_value)
            };
            builder.field("default_value", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl Field {
    ///Returns the enum value of `kind`, or the default if the field is set to an invalid enum value.
    pub fn kind(&self) -> field::Kind {
        field::Kind::from_i32(self.kind).unwrap_or(field::Kind::default())
    }
    ///Sets `kind` to the provided enum value.
    pub fn set_kind(&mut self, value: field::Kind) {
        self.kind = value as i32;
    }
    ///Returns the enum value of `cardinality`, or the default if the field is set to an invalid enum value.
    pub fn cardinality(&self) -> field::Cardinality {
        field::Cardinality::from_i32(self.cardinality).unwrap_or(field::Cardinality::default())
    }
    ///Sets `cardinality` to the provided enum value.
    pub fn set_cardinality(&mut self, value: field::Cardinality) {
        self.cardinality = value as i32;
    }
}
/// Nested message and enum types in `Field`.
pub mod field {
    /// Basic field types.
    #[repr(i32)]
    pub enum Kind {
        /// Field type unknown.
        TypeUnknown = 0,
        /// Field type double.
        TypeDouble = 1,
        /// Field type float.
        TypeFloat = 2,
        /// Field type int64.
        TypeInt64 = 3,
        /// Field type uint64.
        TypeUint64 = 4,
        /// Field type int32.
        TypeInt32 = 5,
        /// Field type fixed64.
        TypeFixed64 = 6,
        /// Field type fixed32.
        TypeFixed32 = 7,
        /// Field type bool.
        TypeBool = 8,
        /// Field type string.
        TypeString = 9,
        /// Field type group. Proto2 syntax only, and deprecated.
        TypeGroup = 10,
        /// Field type message.
        TypeMessage = 11,
        /// Field type bytes.
        TypeBytes = 12,
        /// Field type uint32.
        TypeUint32 = 13,
        /// Field type enum.
        TypeEnum = 14,
        /// Field type sfixed32.
        TypeSfixed32 = 15,
        /// Field type sfixed64.
        TypeSfixed64 = 16,
        /// Field type sint32.
        TypeSint32 = 17,
        /// Field type sint64.
        TypeSint64 = 18,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Kind {
        #[inline]
        fn clone(&self) -> Kind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Kind {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Kind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Kind::TypeUnknown => ::core::fmt::Formatter::write_str(f, "TypeUnknown"),
                Kind::TypeDouble => ::core::fmt::Formatter::write_str(f, "TypeDouble"),
                Kind::TypeFloat => ::core::fmt::Formatter::write_str(f, "TypeFloat"),
                Kind::TypeInt64 => ::core::fmt::Formatter::write_str(f, "TypeInt64"),
                Kind::TypeUint64 => ::core::fmt::Formatter::write_str(f, "TypeUint64"),
                Kind::TypeInt32 => ::core::fmt::Formatter::write_str(f, "TypeInt32"),
                Kind::TypeFixed64 => ::core::fmt::Formatter::write_str(f, "TypeFixed64"),
                Kind::TypeFixed32 => ::core::fmt::Formatter::write_str(f, "TypeFixed32"),
                Kind::TypeBool => ::core::fmt::Formatter::write_str(f, "TypeBool"),
                Kind::TypeString => ::core::fmt::Formatter::write_str(f, "TypeString"),
                Kind::TypeGroup => ::core::fmt::Formatter::write_str(f, "TypeGroup"),
                Kind::TypeMessage => ::core::fmt::Formatter::write_str(f, "TypeMessage"),
                Kind::TypeBytes => ::core::fmt::Formatter::write_str(f, "TypeBytes"),
                Kind::TypeUint32 => ::core::fmt::Formatter::write_str(f, "TypeUint32"),
                Kind::TypeEnum => ::core::fmt::Formatter::write_str(f, "TypeEnum"),
                Kind::TypeSfixed32 => ::core::fmt::Formatter::write_str(f, "TypeSfixed32"),
                Kind::TypeSfixed64 => ::core::fmt::Formatter::write_str(f, "TypeSfixed64"),
                Kind::TypeSint32 => ::core::fmt::Formatter::write_str(f, "TypeSint32"),
                Kind::TypeSint64 => ::core::fmt::Formatter::write_str(f, "TypeSint64"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Kind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Kind {
        #[inline]
        fn eq(&self, other: &Kind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Kind {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Kind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Kind {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Kind {
        #[inline]
        fn partial_cmp(&self, other: &Kind) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Kind {
        #[inline]
        fn cmp(&self, other: &Kind) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    impl Kind {
        ///Returns `true` if `value` is a variant of `Kind`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                3 => true,
                4 => true,
                5 => true,
                6 => true,
                7 => true,
                8 => true,
                9 => true,
                10 => true,
                11 => true,
                12 => true,
                13 => true,
                14 => true,
                15 => true,
                16 => true,
                17 => true,
                18 => true,
                _ => false,
            }
        }
        ///Converts an `i32` to a `Kind`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<Kind> {
            match value {
                0 => ::core::option::Option::Some(Kind::TypeUnknown),
                1 => ::core::option::Option::Some(Kind::TypeDouble),
                2 => ::core::option::Option::Some(Kind::TypeFloat),
                3 => ::core::option::Option::Some(Kind::TypeInt64),
                4 => ::core::option::Option::Some(Kind::TypeUint64),
                5 => ::core::option::Option::Some(Kind::TypeInt32),
                6 => ::core::option::Option::Some(Kind::TypeFixed64),
                7 => ::core::option::Option::Some(Kind::TypeFixed32),
                8 => ::core::option::Option::Some(Kind::TypeBool),
                9 => ::core::option::Option::Some(Kind::TypeString),
                10 => ::core::option::Option::Some(Kind::TypeGroup),
                11 => ::core::option::Option::Some(Kind::TypeMessage),
                12 => ::core::option::Option::Some(Kind::TypeBytes),
                13 => ::core::option::Option::Some(Kind::TypeUint32),
                14 => ::core::option::Option::Some(Kind::TypeEnum),
                15 => ::core::option::Option::Some(Kind::TypeSfixed32),
                16 => ::core::option::Option::Some(Kind::TypeSfixed64),
                17 => ::core::option::Option::Some(Kind::TypeSint32),
                18 => ::core::option::Option::Some(Kind::TypeSint64),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for Kind {
        fn default() -> Kind {
            Kind::TypeUnknown
        }
    }
    impl ::core::convert::From<Kind> for i32 {
        fn from(value: Kind) -> i32 {
            value as i32
        }
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::TypeUnknown => "TYPE_UNKNOWN",
                Kind::TypeDouble => "TYPE_DOUBLE",
                Kind::TypeFloat => "TYPE_FLOAT",
                Kind::TypeInt64 => "TYPE_INT64",
                Kind::TypeUint64 => "TYPE_UINT64",
                Kind::TypeInt32 => "TYPE_INT32",
                Kind::TypeFixed64 => "TYPE_FIXED64",
                Kind::TypeFixed32 => "TYPE_FIXED32",
                Kind::TypeBool => "TYPE_BOOL",
                Kind::TypeString => "TYPE_STRING",
                Kind::TypeGroup => "TYPE_GROUP",
                Kind::TypeMessage => "TYPE_MESSAGE",
                Kind::TypeBytes => "TYPE_BYTES",
                Kind::TypeUint32 => "TYPE_UINT32",
                Kind::TypeEnum => "TYPE_ENUM",
                Kind::TypeSfixed32 => "TYPE_SFIXED32",
                Kind::TypeSfixed64 => "TYPE_SFIXED64",
                Kind::TypeSint32 => "TYPE_SINT32",
                Kind::TypeSint64 => "TYPE_SINT64",
            }
        }
    }
    /// Whether a field is optional, required, or repeated.
    #[repr(i32)]
    pub enum Cardinality {
        /// For fields with unknown cardinality.
        Unknown = 0,
        /// For optional fields.
        Optional = 1,
        /// For required fields. Proto2 syntax only.
        Required = 2,
        /// For repeated fields.
        Repeated = 3,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Cardinality {
        #[inline]
        fn clone(&self) -> Cardinality {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Cardinality {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Cardinality {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Cardinality::Unknown => ::core::fmt::Formatter::write_str(f, "Unknown"),
                Cardinality::Optional => ::core::fmt::Formatter::write_str(f, "Optional"),
                Cardinality::Required => ::core::fmt::Formatter::write_str(f, "Required"),
                Cardinality::Repeated => ::core::fmt::Formatter::write_str(f, "Repeated"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Cardinality {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Cardinality {
        #[inline]
        fn eq(&self, other: &Cardinality) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Cardinality {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Cardinality {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Cardinality {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Cardinality {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Cardinality,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Cardinality {
        #[inline]
        fn cmp(&self, other: &Cardinality) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    impl Cardinality {
        ///Returns `true` if `value` is a variant of `Cardinality`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                3 => true,
                _ => false,
            }
        }
        ///Converts an `i32` to a `Cardinality`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<Cardinality> {
            match value {
                0 => ::core::option::Option::Some(Cardinality::Unknown),
                1 => ::core::option::Option::Some(Cardinality::Optional),
                2 => ::core::option::Option::Some(Cardinality::Required),
                3 => ::core::option::Option::Some(Cardinality::Repeated),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for Cardinality {
        fn default() -> Cardinality {
            Cardinality::Unknown
        }
    }
    impl ::core::convert::From<Cardinality> for i32 {
        fn from(value: Cardinality) -> i32 {
            value as i32
        }
    }
    impl Cardinality {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Cardinality::Unknown => "CARDINALITY_UNKNOWN",
                Cardinality::Optional => "CARDINALITY_OPTIONAL",
                Cardinality::Required => "CARDINALITY_REQUIRED",
                Cardinality::Repeated => "CARDINALITY_REPEATED",
            }
        }
    }
}
/// Enum type definition.
pub struct Enum {
    /// Enum type name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Enum value definitions.
    #[prost(message, repeated, tag = "2")]
    pub enumvalue: ::prost::alloc::vec::Vec<EnumValue>,
    /// Protocol buffer options.
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// The source context.
    #[prost(message, optional, tag = "4")]
    pub source_context: ::core::option::Option<SourceContext>,
    /// The source syntax.
    #[prost(enumeration = "Syntax", tag = "5")]
    pub syntax: i32,
}
#[automatically_derived]
impl ::core::clone::Clone for Enum {
    #[inline]
    fn clone(&self) -> Enum {
        Enum {
            name: ::core::clone::Clone::clone(&self.name),
            enumvalue: ::core::clone::Clone::clone(&self.enumvalue),
            options: ::core::clone::Clone::clone(&self.options),
            source_context: ::core::clone::Clone::clone(&self.source_context),
            syntax: ::core::clone::Clone::clone(&self.syntax),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Enum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Enum {
    #[inline]
    fn eq(&self, other: &Enum) -> bool {
        self.name == other.name
            && self.enumvalue == other.enumvalue
            && self.options == other.options
            && self.source_context == other.source_context
            && self.syntax == other.syntax
    }
}
impl ::prost::Message for Enum {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.name, buf);
        }
        for msg in &self.enumvalue {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
        for msg in &self.options {
            ::prost::encoding::message::encode(3u32, msg, buf);
        }
        if let Some(ref msg) = self.source_context {
            ::prost::encoding::message::encode(4u32, msg, buf);
        }
        if self.syntax != Syntax::default() as i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufEnum,
                _,
                _,
            >(5u32, &self.syntax, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Enum";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.enumvalue;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "enumvalue");
                        error
                    },
                )
            }
            3u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "options");
                        error
                    },
                )
            }
            4u32 => {
                let mut value = &mut self.source_context;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "source_context");
                    error
                })
            }
            5u32 => {
                let mut value = &mut self.syntax;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "syntax");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.name != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.name)
        } else {
            0
        } + ::prost::encoding::message::encoded_len_repeated(2u32, &self.enumvalue)
            + ::prost::encoding::message::encoded_len_repeated(3u32, &self.options)
            + self
                .source_context
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(4u32, msg))
            + if self.syntax != Syntax::default() as i32 {
                ::prost::encoding::int32::encoded_len(5u32, &self.syntax)
            } else {
                0
            }
    }
    fn clear(&mut self) {
        self.name.clear();
        self.enumvalue.clear();
        self.options.clear();
        self.source_context = ::core::option::Option::None;
        self.syntax = Syntax::default() as i32;
    }
}
impl ::core::default::Default for Enum {
    fn default() -> Self {
        Enum {
            name: ::prost::alloc::string::String::new(),
            enumvalue: ::core::default::Default::default(),
            options: ::core::default::Default::default(),
            source_context: ::core::default::Default::default(),
            syntax: Syntax::default() as i32,
        }
    }
}
impl ::core::fmt::Debug for Enum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Enum");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = &self.enumvalue;
            builder.field("enumvalue", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = &self.source_context;
            builder.field("source_context", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a i32);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match Syntax::from_i32(*self.0) {
                            None => ::core::fmt::Debug::fmt(&self.0, f),
                            Some(en) => ::core::fmt::Debug::fmt(&en, f),
                        }
                    }
                }
                ScalarWrapper(&self.syntax)
            };
            builder.field("syntax", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl Enum {
    ///Returns the enum value of `syntax`, or the default if the field is set to an invalid enum value.
    pub fn syntax(&self) -> Syntax {
        Syntax::from_i32(self.syntax).unwrap_or(Syntax::default())
    }
    ///Sets `syntax` to the provided enum value.
    pub fn set_syntax(&mut self, value: Syntax) {
        self.syntax = value as i32;
    }
}
/// Enum value definition.
pub struct EnumValue {
    /// Enum value name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Enum value number.
    #[prost(int32, tag = "2")]
    pub number: i32,
    /// Protocol buffer options.
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
}
#[automatically_derived]
impl ::core::clone::Clone for EnumValue {
    #[inline]
    fn clone(&self) -> EnumValue {
        EnumValue {
            name: ::core::clone::Clone::clone(&self.name),
            number: ::core::clone::Clone::clone(&self.number),
            options: ::core::clone::Clone::clone(&self.options),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumValue {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumValue {
    #[inline]
    fn eq(&self, other: &EnumValue) -> bool {
        self.name == other.name && self.number == other.number && self.options == other.options
    }
}
impl ::prost::Message for EnumValue {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.name, buf);
        }
        if self.number != 0i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(2u32, &self.number, buf);
        }
        for msg in &self.options {
            ::prost::encoding::message::encode(3u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "EnumValue";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.number;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "number");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "options");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.name != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.name)
        } else {
            0
        } + if self.number != 0i32 {
            ::prost::encoding::int32::encoded_len(2u32, &self.number)
        } else {
            0
        } + ::prost::encoding::message::encoded_len_repeated(3u32, &self.options)
    }
    fn clear(&mut self) {
        self.name.clear();
        self.number = 0i32;
        self.options.clear();
    }
}
impl ::core::default::Default for EnumValue {
    fn default() -> Self {
        EnumValue {
            name: ::prost::alloc::string::String::new(),
            number: 0i32,
            options: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for EnumValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("EnumValue");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.number)
            };
            builder.field("number", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        builder.finish()
    }
}
/// A protocol buffer option, which can be attached to a message, field,
/// enumeration, etc.
pub struct Option {
    /// The option's name. For protobuf built-in options (options defined in
    /// descriptor.proto), this is the short name. For example, `"map_entry"`.
    /// For custom options, it should be the fully-qualified name. For example,
    /// `"google.api.http"`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The option's value packed in an Any message. If the value is a primitive,
    /// the corresponding wrapper type defined in google/protobuf/wrappers.proto
    /// should be used. If the value is an enum, it should be stored as an int32
    /// value using the google.protobuf.Int32Value type.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Any>,
}
#[automatically_derived]
impl ::core::clone::Clone for Option {
    #[inline]
    fn clone(&self) -> Option {
        Option {
            name: ::core::clone::Clone::clone(&self.name),
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Option {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Option {
    #[inline]
    fn eq(&self, other: &Option) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::prost::Message for Option {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.name, buf);
        }
        if let Some(ref msg) = self.value {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Option";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.value;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "value");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.name != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.name)
        } else {
            0
        } + self
            .value
            .as_ref()
            .map_or(0, |msg| ::prost::encoding::message::encoded_len(2u32, msg))
    }
    fn clear(&mut self) {
        self.name.clear();
        self.value = ::core::option::Option::None;
    }
}
impl ::core::default::Default for Option {
    fn default() -> Self {
        Option {
            name: ::prost::alloc::string::String::new(),
            value: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for Option {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Option");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = &self.value;
            builder.field("value", &wrapper)
        };
        builder.finish()
    }
}
/// The syntax in which a protocol buffer element is defined.
#[repr(i32)]
pub enum Syntax {
    /// Syntax `proto2`.
    Proto2 = 0,
    /// Syntax `proto3`.
    Proto3 = 1,
}
#[automatically_derived]
impl ::core::clone::Clone for Syntax {
    #[inline]
    fn clone(&self) -> Syntax {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Syntax {}
#[automatically_derived]
impl ::core::fmt::Debug for Syntax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Syntax::Proto2 => ::core::fmt::Formatter::write_str(f, "Proto2"),
            Syntax::Proto3 => ::core::fmt::Formatter::write_str(f, "Proto3"),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Syntax {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Syntax {
    #[inline]
    fn eq(&self, other: &Syntax) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Syntax {}
#[automatically_derived]
impl ::core::cmp::Eq for Syntax {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for Syntax {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Syntax {
    #[inline]
    fn partial_cmp(&self, other: &Syntax) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Syntax {
    #[inline]
    fn cmp(&self, other: &Syntax) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
    }
}
impl Syntax {
    ///Returns `true` if `value` is a variant of `Syntax`.
    pub fn is_valid(value: i32) -> bool {
        match value {
            0 => true,
            1 => true,
            _ => false,
        }
    }
    ///Converts an `i32` to a `Syntax`, or `None` if `value` is not a valid variant.
    pub fn from_i32(value: i32) -> ::core::option::Option<Syntax> {
        match value {
            0 => ::core::option::Option::Some(Syntax::Proto2),
            1 => ::core::option::Option::Some(Syntax::Proto3),
            _ => ::core::option::Option::None,
        }
    }
}
impl ::core::default::Default for Syntax {
    fn default() -> Syntax {
        Syntax::Proto2
    }
}
impl ::core::convert::From<Syntax> for i32 {
    fn from(value: Syntax) -> i32 {
        value as i32
    }
}
impl Syntax {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Syntax::Proto2 => "SYNTAX_PROTO2",
            Syntax::Proto3 => "SYNTAX_PROTO3",
        }
    }
}
/// Api is a light-weight descriptor for an API Interface.
///
/// Interfaces are also described as "protocol buffer services" in some contexts,
/// such as by the "service" keyword in a .proto file, but they are different
/// from API Services, which represent a concrete implementation of an interface
/// as opposed to simply a description of methods and bindings. They are also
/// sometimes simply referred to as "APIs" in other contexts, such as the name of
/// this message itself. See <https://cloud.google.com/apis/design/glossary> for
/// detailed terminology.
pub struct Api {
    /// The fully qualified name of this interface, including package name
    /// followed by the interface's simple name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The methods of this interface, in unspecified order.
    #[prost(message, repeated, tag = "2")]
    pub methods: ::prost::alloc::vec::Vec<Method>,
    /// Any metadata attached to the interface.
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// A version string for this interface. If specified, must have the form
    /// `major-version.minor-version`, as in `1.10`. If the minor version is
    /// omitted, it defaults to zero. If the entire version field is empty, the
    /// major version is derived from the package name, as outlined below. If the
    /// field is not empty, the version in the package name will be verified to be
    /// consistent with what is provided here.
    ///
    /// The versioning schema uses [semantic
    /// versioning](<http://semver.org>) where the major version number
    /// indicates a breaking change and the minor version an additive,
    /// non-breaking change. Both version numbers are signals to users
    /// what to expect from different versions, and should be carefully
    /// chosen based on the product plan.
    ///
    /// The major version is also reflected in the package name of the
    /// interface, which must end in `v<major-version>`, as in
    /// `google.feature.v1`. For major versions 0 and 1, the suffix can
    /// be omitted. Zero major versions must only be used for
    /// experimental, non-GA interfaces.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Source context for the protocol buffer service represented by this
    /// message.
    #[prost(message, optional, tag = "5")]
    pub source_context: ::core::option::Option<SourceContext>,
    /// Included interfaces. See \\[Mixin\]\[\\].
    #[prost(message, repeated, tag = "6")]
    pub mixins: ::prost::alloc::vec::Vec<Mixin>,
    /// The source syntax of the service.
    #[prost(enumeration = "Syntax", tag = "7")]
    pub syntax: i32,
}
#[automatically_derived]
impl ::core::clone::Clone for Api {
    #[inline]
    fn clone(&self) -> Api {
        Api {
            name: ::core::clone::Clone::clone(&self.name),
            methods: ::core::clone::Clone::clone(&self.methods),
            options: ::core::clone::Clone::clone(&self.options),
            version: ::core::clone::Clone::clone(&self.version),
            source_context: ::core::clone::Clone::clone(&self.source_context),
            mixins: ::core::clone::Clone::clone(&self.mixins),
            syntax: ::core::clone::Clone::clone(&self.syntax),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Api {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Api {
    #[inline]
    fn eq(&self, other: &Api) -> bool {
        self.name == other.name
            && self.methods == other.methods
            && self.options == other.options
            && self.version == other.version
            && self.source_context == other.source_context
            && self.mixins == other.mixins
            && self.syntax == other.syntax
    }
}
impl ::prost::Message for Api {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.name, buf);
        }
        for msg in &self.methods {
            ::prost::encoding::message::encode(2u32, msg, buf);
        }
        for msg in &self.options {
            ::prost::encoding::message::encode(3u32, msg, buf);
        }
        if self.version != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(4u32, &self.version, buf);
        }
        if let Some(ref msg) = self.source_context {
            ::prost::encoding::message::encode(5u32, msg, buf);
        }
        for msg in &self.mixins {
            ::prost::encoding::message::encode(6u32, msg, buf);
        }
        if self.syntax != Syntax::default() as i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufEnum,
                _,
                _,
            >(7u32, &self.syntax, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Api";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.methods;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "methods");
                        error
                    },
                )
            }
            3u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "options");
                        error
                    },
                )
            }
            4u32 => {
                let mut value = &mut self.version;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "version");
                    error
                })
            }
            5u32 => {
                let mut value = &mut self.source_context;
                ::prost::encoding::message::merge(
                    wire_type,
                    value.get_or_insert_with(::core::default::Default::default),
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "source_context");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.mixins;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "mixins");
                        error
                    },
                )
            }
            7u32 => {
                let mut value = &mut self.syntax;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "syntax");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.name != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.name)
        } else {
            0
        } + ::prost::encoding::message::encoded_len_repeated(2u32, &self.methods)
            + ::prost::encoding::message::encoded_len_repeated(3u32, &self.options)
            + if self.version != "" {
                ::prost::encoding::string::encoded_len(4u32, &self.version)
            } else {
                0
            }
            + self
                .source_context
                .as_ref()
                .map_or(0, |msg| ::prost::encoding::message::encoded_len(5u32, msg))
            + ::prost::encoding::message::encoded_len_repeated(6u32, &self.mixins)
            + if self.syntax != Syntax::default() as i32 {
                ::prost::encoding::int32::encoded_len(7u32, &self.syntax)
            } else {
                0
            }
    }
    fn clear(&mut self) {
        self.name.clear();
        self.methods.clear();
        self.options.clear();
        self.version.clear();
        self.source_context = ::core::option::Option::None;
        self.mixins.clear();
        self.syntax = Syntax::default() as i32;
    }
}
impl ::core::default::Default for Api {
    fn default() -> Self {
        Api {
            name: ::prost::alloc::string::String::new(),
            methods: ::core::default::Default::default(),
            options: ::core::default::Default::default(),
            version: ::prost::alloc::string::String::new(),
            source_context: ::core::default::Default::default(),
            mixins: ::core::default::Default::default(),
            syntax: Syntax::default() as i32,
        }
    }
}
impl ::core::fmt::Debug for Api {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Api");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = &self.methods;
            builder.field("methods", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.version)
            };
            builder.field("version", &wrapper)
        };
        let builder = {
            let wrapper = &self.source_context;
            builder.field("source_context", &wrapper)
        };
        let builder = {
            let wrapper = &self.mixins;
            builder.field("mixins", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a i32);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match Syntax::from_i32(*self.0) {
                            None => ::core::fmt::Debug::fmt(&self.0, f),
                            Some(en) => ::core::fmt::Debug::fmt(&en, f),
                        }
                    }
                }
                ScalarWrapper(&self.syntax)
            };
            builder.field("syntax", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl Api {
    ///Returns the enum value of `syntax`, or the default if the field is set to an invalid enum value.
    pub fn syntax(&self) -> Syntax {
        Syntax::from_i32(self.syntax).unwrap_or(Syntax::default())
    }
    ///Sets `syntax` to the provided enum value.
    pub fn set_syntax(&mut self, value: Syntax) {
        self.syntax = value as i32;
    }
}
/// Method represents a method of an API interface.
pub struct Method {
    /// The simple name of this method.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A URL of the input message type.
    #[prost(string, tag = "2")]
    pub request_type_url: ::prost::alloc::string::String,
    /// If true, the request is streamed.
    #[prost(bool, tag = "3")]
    pub request_streaming: bool,
    /// The URL of the output message type.
    #[prost(string, tag = "4")]
    pub response_type_url: ::prost::alloc::string::String,
    /// If true, the response is streamed.
    #[prost(bool, tag = "5")]
    pub response_streaming: bool,
    /// Any metadata attached to the method.
    #[prost(message, repeated, tag = "6")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// The source syntax of this method.
    #[prost(enumeration = "Syntax", tag = "7")]
    pub syntax: i32,
}
#[automatically_derived]
impl ::core::clone::Clone for Method {
    #[inline]
    fn clone(&self) -> Method {
        Method {
            name: ::core::clone::Clone::clone(&self.name),
            request_type_url: ::core::clone::Clone::clone(&self.request_type_url),
            request_streaming: ::core::clone::Clone::clone(&self.request_streaming),
            response_type_url: ::core::clone::Clone::clone(&self.response_type_url),
            response_streaming: ::core::clone::Clone::clone(&self.response_streaming),
            options: ::core::clone::Clone::clone(&self.options),
            syntax: ::core::clone::Clone::clone(&self.syntax),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Method {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Method {
    #[inline]
    fn eq(&self, other: &Method) -> bool {
        self.name == other.name
            && self.request_type_url == other.request_type_url
            && self.request_streaming == other.request_streaming
            && self.response_type_url == other.response_type_url
            && self.response_streaming == other.response_streaming
            && self.options == other.options
            && self.syntax == other.syntax
    }
}
impl ::prost::Message for Method {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.name, buf);
        }
        if self.request_type_url != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(2u32, &self.request_type_url, buf);
        }
        if self.request_streaming != false {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufBool,
                _,
                _,
            >(3u32, &self.request_streaming, buf);
        }
        if self.response_type_url != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(4u32, &self.response_type_url, buf);
        }
        if self.response_streaming != false {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufBool,
                _,
                _,
            >(5u32, &self.response_streaming, buf);
        }
        for msg in &self.options {
            ::prost::encoding::message::encode(6u32, msg, buf);
        }
        if self.syntax != Syntax::default() as i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufEnum,
                _,
                _,
            >(7u32, &self.syntax, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Method";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.request_type_url;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "request_type_url");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.request_streaming;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "request_streaming");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.response_type_url;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "response_type_url");
                    error
                })
            }
            5u32 => {
                let mut value = &mut self.response_streaming;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufBool,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "response_streaming");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.options;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "options");
                        error
                    },
                )
            }
            7u32 => {
                let mut value = &mut self.syntax;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufEnum,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "syntax");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.name != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.name)
        } else {
            0
        } + if self.request_type_url != "" {
            ::prost::encoding::string::encoded_len(2u32, &self.request_type_url)
        } else {
            0
        } + if self.request_streaming != false {
            ::prost::encoding::bool::encoded_len(3u32, &self.request_streaming)
        } else {
            0
        } + if self.response_type_url != "" {
            ::prost::encoding::string::encoded_len(4u32, &self.response_type_url)
        } else {
            0
        } + if self.response_streaming != false {
            ::prost::encoding::bool::encoded_len(5u32, &self.response_streaming)
        } else {
            0
        } + ::prost::encoding::message::encoded_len_repeated(6u32, &self.options)
            + if self.syntax != Syntax::default() as i32 {
                ::prost::encoding::int32::encoded_len(7u32, &self.syntax)
            } else {
                0
            }
    }
    fn clear(&mut self) {
        self.name.clear();
        self.request_type_url.clear();
        self.request_streaming = false;
        self.response_type_url.clear();
        self.response_streaming = false;
        self.options.clear();
        self.syntax = Syntax::default() as i32;
    }
}
impl ::core::default::Default for Method {
    fn default() -> Self {
        Method {
            name: ::prost::alloc::string::String::new(),
            request_type_url: ::prost::alloc::string::String::new(),
            request_streaming: false,
            response_type_url: ::prost::alloc::string::String::new(),
            response_streaming: false,
            options: ::core::default::Default::default(),
            syntax: Syntax::default() as i32,
        }
    }
}
impl ::core::fmt::Debug for Method {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Method");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.request_type_url)
            };
            builder.field("request_type_url", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.request_streaming)
            };
            builder.field("request_streaming", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.response_type_url)
            };
            builder.field("response_type_url", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.response_streaming)
            };
            builder.field("response_streaming", &wrapper)
        };
        let builder = {
            let wrapper = &self.options;
            builder.field("options", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a i32);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match Syntax::from_i32(*self.0) {
                            None => ::core::fmt::Debug::fmt(&self.0, f),
                            Some(en) => ::core::fmt::Debug::fmt(&en, f),
                        }
                    }
                }
                ScalarWrapper(&self.syntax)
            };
            builder.field("syntax", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl Method {
    ///Returns the enum value of `syntax`, or the default if the field is set to an invalid enum value.
    pub fn syntax(&self) -> Syntax {
        Syntax::from_i32(self.syntax).unwrap_or(Syntax::default())
    }
    ///Sets `syntax` to the provided enum value.
    pub fn set_syntax(&mut self, value: Syntax) {
        self.syntax = value as i32;
    }
}
/// Declares an API Interface to be included in this interface. The including
/// interface must redeclare all the methods from the included interface, but
/// documentation and options are inherited as follows:
///
/// * If after comment and whitespace stripping, the documentation
///   string of the redeclared method is empty, it will be inherited
///   from the original method.
///
/// * Each annotation belonging to the service config (http,
///   visibility) which is not set in the redeclared method will be
///   inherited.
///
/// * If an http annotation is inherited, the path pattern will be
///   modified as follows. Any version prefix will be replaced by the
///   version of the including interface plus the \\[root\]\[\\] path if
///   specified.
///
/// Example of a simple mixin:
///
/// ```text
/// package google.acl.v1;
/// service AccessControl {
///    // Get the underlying ACL object.
///    rpc GetAcl(GetAclRequest) returns (Acl) {
///      option (google.api.http).get = "/v1/{resource=**}:getAcl";
///    }
/// }
///
/// package google.storage.v2;
/// service Storage {
///    rpc GetAcl(GetAclRequest) returns (Acl);
///
///    // Get a data record.
///    rpc GetData(GetDataRequest) returns (Data) {
///      option (google.api.http).get = "/v2/{resource=**}";
///    }
/// }
/// ```
///
/// Example of a mixin configuration:
///
/// ```text
/// apis:
/// - name: google.storage.v2.Storage
///    mixins:
///    - name: google.acl.v1.AccessControl
/// ```
///
/// The mixin construct implies that all methods in `AccessControl` are
/// also declared with same name and request/response types in
/// `Storage`. A documentation generator or annotation processor will
/// see the effective `Storage.GetAcl` method after inheriting
/// documentation and annotations as follows:
///
/// ```text
/// service Storage {
///    // Get the underlying ACL object.
///    rpc GetAcl(GetAclRequest) returns (Acl) {
///      option (google.api.http).get = "/v2/{resource=**}:getAcl";
///    }
///    ...
/// }
/// ```
///
/// Note how the version in the path pattern changed from `v1` to `v2`.
///
/// If the `root` field in the mixin is specified, it should be a
/// relative path under which inherited HTTP paths are placed. Example:
///
/// ```text
/// apis:
/// - name: google.storage.v2.Storage
///    mixins:
///    - name: google.acl.v1.AccessControl
///      root: acls
/// ```
///
/// This implies the following inherited HTTP annotation:
///
/// ```text
/// service Storage {
///    // Get the underlying ACL object.
///    rpc GetAcl(GetAclRequest) returns (Acl) {
///      option (google.api.http).get = "/v2/acls/{resource=**}:getAcl";
///    }
///    ...
/// }
/// ```
pub struct Mixin {
    /// The fully qualified name of the interface which is included.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If non-empty specifies a path under which inherited HTTP paths
    /// are rooted.
    #[prost(string, tag = "2")]
    pub root: ::prost::alloc::string::String,
}
#[automatically_derived]
impl ::core::clone::Clone for Mixin {
    #[inline]
    fn clone(&self) -> Mixin {
        Mixin {
            name: ::core::clone::Clone::clone(&self.name),
            root: ::core::clone::Clone::clone(&self.root),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Mixin {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Mixin {
    #[inline]
    fn eq(&self, other: &Mixin) -> bool {
        self.name == other.name && self.root == other.root
    }
}
impl ::prost::Message for Mixin {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.name != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.name, buf);
        }
        if self.root != "" {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(2u32, &self.root, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Mixin";
        match tag {
            1u32 => {
                let mut value = &mut self.name;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "name");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.root;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "root");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.name != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.name)
        } else {
            0
        } + if self.root != "" {
            ::prost::encoding::string::encoded_len(2u32, &self.root)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.name.clear();
        self.root.clear();
    }
}
impl ::core::default::Default for Mixin {
    fn default() -> Self {
        Mixin {
            name: ::prost::alloc::string::String::new(),
            root: ::prost::alloc::string::String::new(),
        }
    }
}
impl ::core::fmt::Debug for Mixin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Mixin");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name)
            };
            builder.field("name", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.root)
            };
            builder.field("root", &wrapper)
        };
        builder.finish()
    }
}
/// A Duration represents a signed, fixed-length span of time represented
/// as a count of seconds and fractions of seconds at nanosecond
/// resolution. It is independent of any calendar and concepts like "day"
/// or "month". It is related to Timestamp in that the difference between
/// two Timestamp values is a Duration and it can be added or subtracted
/// from a Timestamp. Range is approximately +-10,000 years.
///
/// # Examples
///
/// Example 1: Compute Duration from two Timestamps in pseudo code.
///
/// ```text
/// Timestamp start = ...;
/// Timestamp end = ...;
/// Duration duration = ...;
///
/// duration.seconds = end.seconds - start.seconds;
/// duration.nanos = end.nanos - start.nanos;
///
/// if (duration.seconds < 0 && duration.nanos > 0) {
///    duration.seconds += 1;
///    duration.nanos -= 1000000000;
/// } else if (duration.seconds > 0 && duration.nanos < 0) {
///    duration.seconds -= 1;
///    duration.nanos += 1000000000;
/// }
/// ```
///
/// Example 2: Compute Timestamp from Timestamp + Duration in pseudo code.
///
/// ```text
/// Timestamp start = ...;
/// Duration duration = ...;
/// Timestamp end = ...;
///
/// end.seconds = start.seconds + duration.seconds;
/// end.nanos = start.nanos + duration.nanos;
///
/// if (end.nanos < 0) {
///    end.seconds -= 1;
///    end.nanos += 1000000000;
/// } else if (end.nanos >= 1000000000) {
///    end.seconds += 1;
///    end.nanos -= 1000000000;
/// }
/// ```
///
/// Example 3: Compute Duration from datetime.timedelta in Python.
///
/// ```text
/// td = datetime.timedelta(days=3, minutes=10)
/// duration = Duration()
/// duration.FromTimedelta(td)
/// ```
///
/// # JSON Mapping
///
/// In JSON format, the Duration type is encoded as a string rather than an
/// object, where the string ends in the suffix "s" (indicating seconds) and
/// is preceded by the number of seconds, with nanoseconds expressed as
/// fractional seconds. For example, 3 seconds with 0 nanoseconds should be
/// encoded in JSON format as "3s", while 3 seconds and 1 nanosecond should
/// be expressed in JSON format as "3.000000001s", and 3 seconds and 1
/// microsecond should be expressed in JSON format as "3.000001s".
pub struct Duration {
    /// Signed seconds of the span of time. Must be from -315,576,000,000
    /// to +315,576,000,000 inclusive. Note: these bounds are computed from:
    /// 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Signed fractions of a second at nanosecond resolution of the span
    /// of time. Durations less than one second are represented with a 0
    /// `seconds` field and a positive or negative `nanos` field. For durations
    /// of one second or more, a non-zero value for the `nanos` field must be
    /// of the same sign as the `seconds` field. Must be from -999,999,999
    /// to +999,999,999 inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
#[automatically_derived]
impl ::core::clone::Clone for Duration {
    #[inline]
    fn clone(&self) -> Duration {
        Duration {
            seconds: ::core::clone::Clone::clone(&self.seconds),
            nanos: ::core::clone::Clone::clone(&self.nanos),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Duration {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Duration {
    #[inline]
    fn eq(&self, other: &Duration) -> bool {
        self.seconds == other.seconds && self.nanos == other.nanos
    }
}
impl ::prost::Message for Duration {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.seconds != 0i64 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int64,
                _,
                _,
            >(1u32, &self.seconds, buf);
        }
        if self.nanos != 0i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(2u32, &self.nanos, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Duration";
        match tag {
            1u32 => {
                let mut value = &mut self.seconds;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int64,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "seconds");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.nanos;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "nanos");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.seconds != 0i64 {
            ::prost::encoding::int64::encoded_len(1u32, &self.seconds)
        } else {
            0
        } + if self.nanos != 0i32 {
            ::prost::encoding::int32::encoded_len(2u32, &self.nanos)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.seconds = 0i64;
        self.nanos = 0i32;
    }
}
impl ::core::default::Default for Duration {
    fn default() -> Self {
        Duration {
            seconds: 0i64,
            nanos: 0i32,
        }
    }
}
impl ::core::fmt::Debug for Duration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Duration");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.seconds)
            };
            builder.field("seconds", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.nanos)
            };
            builder.field("nanos", &wrapper)
        };
        builder.finish()
    }
}
/// `FieldMask` represents a set of symbolic field paths, for example:
///
/// ```text
/// paths: "f.a"
/// paths: "f.b.d"
/// ```
///
/// Here `f` represents a field in some root message, `a` and `b`
/// fields in the message found in `f`, and `d` a field found in the
/// message in `f.b`.
///
/// Field masks are used to specify a subset of fields that should be
/// returned by a get operation or modified by an update operation.
/// Field masks also have a custom JSON encoding (see below).
///
/// # Field Masks in Projections
///
/// When used in the context of a projection, a response message or
/// sub-message is filtered by the API to only contain those fields as
/// specified in the mask. For example, if the mask in the previous
/// example is applied to a response message as follows:
///
/// ```text
/// f {
///    a : 22
///    b {
///      d : 1
///      x : 2
///    }
///    y : 13
/// }
/// z: 8
/// ```
///
/// The result will not contain specific values for fields x,y and z
/// (their value will be set to the default, and omitted in proto text
/// output):
///
/// ```text
/// f {
///    a : 22
///    b {
///      d : 1
///    }
/// }
/// ```
///
/// A repeated field is not allowed except at the last position of a
/// paths string.
///
/// If a FieldMask object is not present in a get operation, the
/// operation applies to all fields (as if a FieldMask of all fields
/// had been specified).
///
/// Note that a field mask does not necessarily apply to the
/// top-level response message. In case of a REST get operation, the
/// field mask applies directly to the response, but in case of a REST
/// list operation, the mask instead applies to each individual message
/// in the returned resource list. In case of a REST custom method,
/// other definitions may be used. Where the mask applies will be
/// clearly documented together with its declaration in the API.  In
/// any case, the effect on the returned resource/resources is required
/// behavior for APIs.
///
/// # Field Masks in Update Operations
///
/// A field mask in update operations specifies which fields of the
/// targeted resource are going to be updated. The API is required
/// to only change the values of the fields as specified in the mask
/// and leave the others untouched. If a resource is passed in to
/// describe the updated values, the API ignores the values of all
/// fields not covered by the mask.
///
/// If a repeated field is specified for an update operation, new values will
/// be appended to the existing repeated field in the target resource. Note that
/// a repeated field is only allowed in the last position of a `paths` string.
///
/// If a sub-message is specified in the last position of the field mask for an
/// update operation, then new value will be merged into the existing sub-message
/// in the target resource.
///
/// For example, given the target message:
///
/// ```text
/// f {
///    b {
///      d: 1
///      x: 2
///    }
///    c: \[1\]
/// }
/// ```
///
/// And an update message:
///
/// ```text
/// f {
///    b {
///      d: 10
///    }
///    c: \[2\]
/// }
/// ```
///
/// then if the field mask is:
///
/// paths: \["f.b", "f.c"\]
///
/// then the result will be:
///
/// ```text
/// f {
///    b {
///      d: 10
///      x: 2
///    }
///    c: [1, 2]
/// }
/// ```
///
/// An implementation may provide options to override this default behavior for
/// repeated and message fields.
///
/// In order to reset a field's value to the default, the field must
/// be in the mask and set to the default value in the provided resource.
/// Hence, in order to reset all fields of a resource, provide a default
/// instance of the resource and set all fields in the mask, or do
/// not provide a mask as described below.
///
/// If a field mask is not present on update, the operation applies to
/// all fields (as if a field mask of all fields has been specified).
/// Note that in the presence of schema evolution, this may mean that
/// fields the client does not know and has therefore not filled into
/// the request will be reset to their default. If this is unwanted
/// behavior, a specific service may require a client to always specify
/// a field mask, producing an error if not.
///
/// As with get operations, the location of the resource which
/// describes the updated values in the request message depends on the
/// operation kind. In any case, the effect of the field mask is
/// required to be honored by the API.
///
/// ## Considerations for HTTP REST
///
/// The HTTP kind of an update operation which uses a field mask must
/// be set to PATCH instead of PUT in order to satisfy HTTP semantics
/// (PUT must only be used for full updates).
///
/// # JSON Encoding of Field Masks
///
/// In JSON, a field mask is encoded as a single string where paths are
/// separated by a comma. Fields name in each path are converted
/// to/from lower-camel naming conventions.
///
/// As an example, consider the following message declarations:
///
/// ```text
/// message Profile {
///    User user = 1;
///    Photo photo = 2;
/// }
/// message User {
///    string display_name = 1;
///    string address = 2;
/// }
/// ```
///
/// In proto a field mask for `Profile` may look as such:
///
/// ```text
/// mask {
///    paths: "user.display_name"
///    paths: "photo"
/// }
/// ```
///
/// In JSON, the same mask is represented as below:
///
/// ```text
/// {
///    mask: "user.displayName,photo"
/// }
/// ```
///
/// # Field Masks and Oneof Fields
///
/// Field masks treat fields in oneofs just as regular fields. Consider the
/// following message:
///
/// ```text
/// message SampleMessage {
///    oneof test_oneof {
///      string name = 4;
///      SubMessage sub_message = 9;
///    }
/// }
/// ```
///
/// The field mask can be:
///
/// ```text
/// mask {
///    paths: "name"
/// }
/// ```
///
/// Or:
///
/// ```text
/// mask {
///    paths: "sub_message"
/// }
/// ```
///
/// Note that oneof type names ("test_oneof" in this case) cannot be used in
/// paths.
///
/// ## Field Mask Verification
///
/// The implementation of any API method which has a FieldMask type field in the
/// request should verify the included field paths, and return an
/// `INVALID_ARGUMENT` error if any path is unmappable.
pub struct FieldMask {
    /// The set of field mask paths.
    #[prost(string, repeated, tag = "1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[automatically_derived]
impl ::core::clone::Clone for FieldMask {
    #[inline]
    fn clone(&self) -> FieldMask {
        FieldMask {
            paths: ::core::clone::Clone::clone(&self.paths),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FieldMask {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FieldMask {
    #[inline]
    fn eq(&self, other: &FieldMask) -> bool {
        self.paths == other.paths
    }
}
impl ::prost::Message for FieldMask {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
            ::prost::encoding_traits::ProtobufString,
            _,
            _,
        >(1u32, &self.paths, buf);
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "FieldMask";
        match tag {
            1u32 => {
                let mut value = &mut self.paths;
                <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "paths");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::string::encoded_len_repeated(1u32, &self.paths)
    }
    fn clear(&mut self) {
        self.paths.clear();
    }
}
impl ::core::default::Default for FieldMask {
    fn default() -> Self {
        FieldMask {
            paths: ::prost::alloc::vec::Vec::new(),
        }
    }
}
impl ::core::fmt::Debug for FieldMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("FieldMask");
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let mut vec_builder = f.debug_list();
                        for v in self.0 {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            vec_builder.entry(&Inner(v));
                        }
                        vec_builder.finish()
                    }
                }
                ScalarWrapper(&self.paths)
            };
            builder.field("paths", &wrapper)
        };
        builder.finish()
    }
}
/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
///
/// The JSON representation for `Struct` is JSON object.
pub struct Struct {
    /// Unordered map of dynamically typed values.
    #[prost(btree_map = "string, message", tag = "1")]
    pub fields: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, Value>,
}
#[automatically_derived]
impl ::core::clone::Clone for Struct {
    #[inline]
    fn clone(&self) -> Struct {
        Struct {
            fields: ::core::clone::Clone::clone(&self.fields),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Struct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Struct {
    #[inline]
    fn eq(&self, other: &Struct) -> bool {
        self.fields == other.fields
    }
}
impl ::prost::Message for Struct {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        ::prost::encoding::btree_map::encode(
            ::prost::encoding::string::encode,
            ::prost::encoding::string::encoded_len,
            ::prost::encoding::message::encode,
            ::prost::encoding::message::encoded_len,
            1u32,
            &self.fields,
            buf,
        );
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Struct";
        match tag {
            1u32 => {
                let mut value = &mut self.fields;
                ::prost::encoding::btree_map::merge(
                    ::prost::encoding::string::merge,
                    ::prost::encoding::message::merge,
                    &mut value,
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "fields");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::btree_map::encoded_len(
            ::prost::encoding::string::encoded_len,
            ::prost::encoding::message::encoded_len,
            1u32,
            &self.fields,
        )
    }
    fn clear(&mut self) {
        self.fields.clear();
    }
}
impl ::core::default::Default for Struct {
    fn default() -> Self {
        Struct {
            fields: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for Struct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Struct");
        let builder = {
            let wrapper = {
                struct MapWrapper<'a, V: 'a>(
                    &'a ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, V>,
                );
                impl<'a, V> ::core::fmt::Debug for MapWrapper<'a, V>
                where
                    V: ::core::fmt::Debug + 'a,
                {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        fn KeyWrapper<T>(v: T) -> T {
                            v
                        }
                        fn ValueWrapper<T>(v: T) -> T {
                            v
                        }
                        let mut builder = f.debug_map();
                        for (k, v) in self.0 {
                            builder.entry(&KeyWrapper(k), &ValueWrapper(v));
                        }
                        builder.finish()
                    }
                }
                MapWrapper(&self.fields)
            };
            builder.field("fields", &wrapper)
        };
        builder.finish()
    }
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of these
/// variants. Absence of any variant indicates an error.
///
/// The JSON representation for `Value` is JSON value.
pub struct Value {
    /// The kind of value.
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<value::Kind>,
}
#[automatically_derived]
impl ::core::clone::Clone for Value {
    #[inline]
    fn clone(&self) -> Value {
        Value {
            kind: ::core::clone::Clone::clone(&self.kind),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Value {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Value {
    #[inline]
    fn eq(&self, other: &Value) -> bool {
        self.kind == other.kind
    }
}
impl ::prost::Message for Value {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if let Some(ref oneof) = self.kind {
            oneof.encode(buf)
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Value";
        match tag {
            1u32 | 2u32 | 3u32 | 4u32 | 5u32 | 6u32 => {
                let mut value = &mut self.kind;
                value::Kind::merge(value, tag, wire_type, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "kind");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + self.kind.as_ref().map_or(0, value::Kind::encoded_len)
    }
    fn clear(&mut self) {
        self.kind = ::core::option::Option::None;
    }
}
impl ::core::default::Default for Value {
    fn default() -> Self {
        Value {
            kind: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Value");
        let builder = {
            let wrapper = &self.kind;
            builder.field("kind", &wrapper)
        };
        builder.finish()
    }
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    pub enum Kind {
        /// Represents a null value.
        #[prost(enumeration = "super::NullValue", tag = "1")]
        NullValue(i32),
        /// Represents a double value.
        #[prost(double, tag = "2")]
        NumberValue(f64),
        /// Represents a string value.
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        /// Represents a boolean value.
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        /// Represents a structured value.
        #[prost(message, tag = "5")]
        StructValue(super::Struct),
        /// Represents a repeated `Value`.
        #[prost(message, tag = "6")]
        ListValue(super::ListValue),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Kind {
        #[inline]
        fn clone(&self) -> Kind {
            match self {
                Kind::NullValue(__self_0) => Kind::NullValue(::core::clone::Clone::clone(__self_0)),
                Kind::NumberValue(__self_0) => {
                    Kind::NumberValue(::core::clone::Clone::clone(__self_0))
                }
                Kind::StringValue(__self_0) => {
                    Kind::StringValue(::core::clone::Clone::clone(__self_0))
                }
                Kind::BoolValue(__self_0) => Kind::BoolValue(::core::clone::Clone::clone(__self_0)),
                Kind::StructValue(__self_0) => {
                    Kind::StructValue(::core::clone::Clone::clone(__self_0))
                }
                Kind::ListValue(__self_0) => Kind::ListValue(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Kind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Kind {
        #[inline]
        fn eq(&self, other: &Kind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Kind::NullValue(__self_0), Kind::NullValue(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Kind::NumberValue(__self_0), Kind::NumberValue(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Kind::StringValue(__self_0), Kind::StringValue(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Kind::BoolValue(__self_0), Kind::BoolValue(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Kind::StructValue(__self_0), Kind::StructValue(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Kind::ListValue(__self_0), Kind::ListValue(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    impl Kind {
        /// Encodes the message to a buffer.
        pub fn encode<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            match *self {
                Kind::NullValue(ref value) => {
                    <::prost::encoding_traits::Required as ::prost::encoding_traits::Mode>::encode::<
                        ::prost::encoding_traits::ProtobufEnum,
                        _,
                        _,
                    >(1u32, &*value, buf);
                }
                Kind::NumberValue(ref value) => {
                    <::prost::encoding_traits::Required as ::prost::encoding_traits::Mode>::encode::<
                        ::prost::encoding_traits::ProtobufDouble,
                        _,
                        _,
                    >(2u32, &*value, buf);
                }
                Kind::StringValue(ref value) => {
                    <::prost::encoding_traits::Required as ::prost::encoding_traits::Mode>::encode::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(3u32, &*value, buf);
                }
                Kind::BoolValue(ref value) => {
                    <::prost::encoding_traits::Required as ::prost::encoding_traits::Mode>::encode::<
                        ::prost::encoding_traits::ProtobufBool,
                        _,
                        _,
                    >(4u32, &*value, buf);
                }
                Kind::StructValue(ref value) => {
                    ::prost::encoding::message::encode(5u32, &*value, buf);
                }
                Kind::ListValue(ref value) => {
                    ::prost::encoding::message::encode(6u32, &*value, buf);
                }
            }
        }
        /// Decodes an instance of the message from a buffer, and merges it into self.
        pub fn merge<B>(
            field: &mut ::core::option::Option<Kind>,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            match tag { 1u32 => { match field { :: core :: option :: Option :: Some (Kind :: NullValue (ref mut value)) => { < :: prost :: encoding_traits :: Required as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufEnum , _ , _ > (wire_type , value , buf , ctx) } _ => { let mut owned_value = :: core :: default :: Default :: default () ; let value = & mut owned_value ; < :: prost :: encoding_traits :: Required as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufEnum , _ , _ > (wire_type , value , buf , ctx) . map (| _ | * field = :: core :: option :: Option :: Some (Kind :: NullValue (owned_value))) } } } 2u32 => { match field { :: core :: option :: Option :: Some (Kind :: NumberValue (ref mut value)) => { < :: prost :: encoding_traits :: Required as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufDouble , _ , _ > (wire_type , value , buf , ctx) } _ => { let mut owned_value = :: core :: default :: Default :: default () ; let value = & mut owned_value ; < :: prost :: encoding_traits :: Required as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufDouble , _ , _ > (wire_type , value , buf , ctx) . map (| _ | * field = :: core :: option :: Option :: Some (Kind :: NumberValue (owned_value))) } } } 3u32 => { match field { :: core :: option :: Option :: Some (Kind :: StringValue (ref mut value)) => { < :: prost :: encoding_traits :: Required as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufString , _ , _ > (wire_type , value , buf , ctx) } _ => { let mut owned_value = :: core :: default :: Default :: default () ; let value = & mut owned_value ; < :: prost :: encoding_traits :: Required as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufString , _ , _ > (wire_type , value , buf , ctx) . map (| _ | * field = :: core :: option :: Option :: Some (Kind :: StringValue (owned_value))) } } } 4u32 => { match field { :: core :: option :: Option :: Some (Kind :: BoolValue (ref mut value)) => { < :: prost :: encoding_traits :: Required as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufBool , _ , _ > (wire_type , value , buf , ctx) } _ => { let mut owned_value = :: core :: default :: Default :: default () ; let value = & mut owned_value ; < :: prost :: encoding_traits :: Required as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufBool , _ , _ > (wire_type , value , buf , ctx) . map (| _ | * field = :: core :: option :: Option :: Some (Kind :: BoolValue (owned_value))) } } } 5u32 => { match field { :: core :: option :: Option :: Some (Kind :: StructValue (ref mut value)) => { :: prost :: encoding :: message :: merge (wire_type , value , buf , ctx) } _ => { let mut owned_value = :: core :: default :: Default :: default () ; let value = & mut owned_value ; :: prost :: encoding :: message :: merge (wire_type , value , buf , ctx) . map (| _ | * field = :: core :: option :: Option :: Some (Kind :: StructValue (owned_value))) } } } 6u32 => { match field { :: core :: option :: Option :: Some (Kind :: ListValue (ref mut value)) => { :: prost :: encoding :: message :: merge (wire_type , value , buf , ctx) } _ => { let mut owned_value = :: core :: default :: Default :: default () ; let value = & mut owned_value ; :: prost :: encoding :: message :: merge (wire_type , value , buf , ctx) . map (| _ | * field = :: core :: option :: Option :: Some (Kind :: ListValue (owned_value))) } } } _ => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["internal error: entered unreachable code: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: core :: fmt :: Arguments :: new_v1 (& ["invalid Kind tag: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& tag)]))])) , }
        }
        /// Returns the encoded length of the message without a length delimiter.
        #[inline]
        pub fn encoded_len(&self) -> usize {
            match *self {
                Kind::NullValue(ref value) => ::prost::encoding::int32::encoded_len(1u32, &*value),
                Kind::NumberValue(ref value) => {
                    ::prost::encoding::double::encoded_len(2u32, &*value)
                }
                Kind::StringValue(ref value) => {
                    ::prost::encoding::string::encoded_len(3u32, &*value)
                }
                Kind::BoolValue(ref value) => ::prost::encoding::bool::encoded_len(4u32, &*value),
                Kind::StructValue(ref value) => {
                    ::prost::encoding::message::encoded_len(5u32, &*value)
                }
                Kind::ListValue(ref value) => {
                    ::prost::encoding::message::encoded_len(6u32, &*value)
                }
            }
        }
    }
    impl ::core::fmt::Debug for Kind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Kind::NullValue(ref value) => {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                match super::NullValue::from_i32(*self.0) {
                                    None => ::core::fmt::Debug::fmt(&self.0, f),
                                    Some(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ScalarWrapper(&*value)
                    };
                    f.debug_tuple("NullValue").field(&wrapper).finish()
                }
                Kind::NumberValue(ref value) => {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&*value)
                    };
                    f.debug_tuple("NumberValue").field(&wrapper).finish()
                }
                Kind::StringValue(ref value) => {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&*value)
                    };
                    f.debug_tuple("StringValue").field(&wrapper).finish()
                }
                Kind::BoolValue(ref value) => {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&*value)
                    };
                    f.debug_tuple("BoolValue").field(&wrapper).finish()
                }
                Kind::StructValue(ref value) => {
                    let wrapper = &*value;
                    f.debug_tuple("StructValue").field(&wrapper).finish()
                }
                Kind::ListValue(ref value) => {
                    let wrapper = &*value;
                    f.debug_tuple("ListValue").field(&wrapper).finish()
                }
            }
        }
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is JSON array.
pub struct ListValue {
    /// Repeated field of dynamically typed values.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[automatically_derived]
impl ::core::clone::Clone for ListValue {
    #[inline]
    fn clone(&self) -> ListValue {
        ListValue {
            values: ::core::clone::Clone::clone(&self.values),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ListValue {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ListValue {
    #[inline]
    fn eq(&self, other: &ListValue) -> bool {
        self.values == other.values
    }
}
impl ::prost::Message for ListValue {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        for msg in &self.values {
            ::prost::encoding::message::encode(1u32, msg, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "ListValue";
        match tag {
            1u32 => {
                let mut value = &mut self.values;
                ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "values");
                        error
                    },
                )
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.values)
    }
    fn clear(&mut self) {
        self.values.clear();
    }
}
impl ::core::default::Default for ListValue {
    fn default() -> Self {
        ListValue {
            values: ::core::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for ListValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("ListValue");
        let builder = {
            let wrapper = &self.values;
            builder.field("values", &wrapper)
        };
        builder.finish()
    }
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.
///
/// The JSON representation for `NullValue` is JSON `null`.
#[repr(i32)]
pub enum NullValue {
    /// Null value.
    NullValue = 0,
}
#[automatically_derived]
impl ::core::clone::Clone for NullValue {
    #[inline]
    fn clone(&self) -> NullValue {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for NullValue {}
#[automatically_derived]
impl ::core::fmt::Debug for NullValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "NullValue")
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for NullValue {}
#[automatically_derived]
impl ::core::cmp::PartialEq for NullValue {
    #[inline]
    fn eq(&self, other: &NullValue) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for NullValue {}
#[automatically_derived]
impl ::core::cmp::Eq for NullValue {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for NullValue {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for NullValue {
    #[inline]
    fn partial_cmp(&self, other: &NullValue) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for NullValue {
    #[inline]
    fn cmp(&self, other: &NullValue) -> ::core::cmp::Ordering {
        ::core::cmp::Ordering::Equal
    }
}
impl NullValue {
    ///Returns `true` if `value` is a variant of `NullValue`.
    pub fn is_valid(value: i32) -> bool {
        match value {
            0 => true,
            _ => false,
        }
    }
    ///Converts an `i32` to a `NullValue`, or `None` if `value` is not a valid variant.
    pub fn from_i32(value: i32) -> ::core::option::Option<NullValue> {
        match value {
            0 => ::core::option::Option::Some(NullValue::NullValue),
            _ => ::core::option::Option::None,
        }
    }
}
impl ::core::default::Default for NullValue {
    fn default() -> NullValue {
        NullValue::NullValue
    }
}
impl ::core::convert::From<NullValue> for i32 {
    fn from(value: NullValue) -> i32 {
        value as i32
    }
}
impl NullValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NullValue::NullValue => "NULL_VALUE",
        }
    }
}
/// A Timestamp represents a point in time independent of any time zone or local
/// calendar, encoded as a count of seconds and fractions of seconds at
/// nanosecond resolution. The count is relative to an epoch at UTC midnight on
/// January 1, 1970, in the proleptic Gregorian calendar which extends the
/// Gregorian calendar backwards to year one.
///
/// All minutes are 60 seconds long. Leap seconds are "smeared" so that no leap
/// second table is needed for interpretation, using a [24-hour linear
/// smear](<https://developers.google.com/time/smear>).
///
/// The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By
/// restricting to that range, we ensure that we can convert to and from [RFC
/// 3339](<https://www.ietf.org/rfc/rfc3339.txt>) date strings.
///
/// # Examples
///
/// Example 1: Compute Timestamp from POSIX `time()`.
///
/// ```text
/// Timestamp timestamp;
/// timestamp.set_seconds(time(NULL));
/// timestamp.set_nanos(0);
/// ```
///
/// Example 2: Compute Timestamp from POSIX `gettimeofday()`.
///
/// ```text
/// struct timeval tv;
/// gettimeofday(&tv, NULL);
///
/// Timestamp timestamp;
/// timestamp.set_seconds(tv.tv_sec);
/// timestamp.set_nanos(tv.tv_usec * 1000);
/// ```
///
/// Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
///
/// ```text
/// FILETIME ft;
/// GetSystemTimeAsFileTime(&ft);
/// UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
///
/// // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
/// // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
/// Timestamp timestamp;
/// timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
/// timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
/// ```
///
/// Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
///
/// ```text
/// long millis = System.currentTimeMillis();
///
/// Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
///      .setNanos((int) ((millis % 1000) * 1000000)).build();
/// ```
///
/// Example 5: Compute Timestamp from Java `Instant.now()`.
///
/// ```text
/// Instant now = Instant.now();
///
/// Timestamp timestamp =
///      Timestamp.newBuilder().setSeconds(now.getEpochSecond())
///          .setNanos(now.getNano()).build();
/// ```
///
/// Example 6: Compute Timestamp from current time in Python.
///
/// ```text
/// timestamp = Timestamp()
/// timestamp.GetCurrentTime()
/// ```
///
/// # JSON Mapping
///
/// In JSON format, the Timestamp type is encoded as a string in the
/// [RFC 3339](<https://www.ietf.org/rfc/rfc3339.txt>) format. That is, the
/// format is "{year}-{month}-{day}T{hour}:{min}:{sec}\\[.{frac_sec}\\]Z"
/// where {year} is always expressed using four digits while {month}, {day},
/// {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
/// seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
/// are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
/// is required. A proto3 JSON serializer should always use UTC (as indicated by
/// "Z") when printing the Timestamp type and a proto3 JSON parser should be
/// able to accept both UTC and other timezones (as indicated by an offset).
///
/// For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
/// 01:30 UTC on January 15, 2017.
///
/// In JavaScript, one can convert a Date object to this format using the
/// standard
/// \[toISOString()\](<https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString>)
/// method. In Python, a standard `datetime.datetime` object can be converted
/// to this format using
/// \[`strftime`\](<https://docs.python.org/2/library/time.html#time.strftime>) with
/// the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use
/// the Joda Time's \[`ISODateTimeFormat.dateTime()`\](<http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime%2D%2D>) to obtain a formatter capable of generating timestamps in this format.
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
#[automatically_derived]
impl ::core::clone::Clone for Timestamp {
    #[inline]
    fn clone(&self) -> Timestamp {
        Timestamp {
            seconds: ::core::clone::Clone::clone(&self.seconds),
            nanos: ::core::clone::Clone::clone(&self.nanos),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Timestamp {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Timestamp {
    #[inline]
    fn eq(&self, other: &Timestamp) -> bool {
        self.seconds == other.seconds && self.nanos == other.nanos
    }
}
impl ::prost::Message for Timestamp {
    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::prost::bytes::BufMut,
    {
        if self.seconds != 0i64 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int64,
                _,
                _,
            >(1u32, &self.seconds, buf);
        }
        if self.nanos != 0i32 {
            <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(2u32, &self.nanos, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError>
    where
        B: ::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = "Timestamp";
        match tag {
            1u32 => {
                let mut value = &mut self.seconds;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int64,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "seconds");
                    error
                })
            }
            2u32 => {
                let mut value = &mut self.nanos;
                <::prost::encoding_traits::Plain as ::prost::encoding_traits::Mode>::merge::<
                    ::prost::encoding_traits::Int32,
                    _,
                    _,
                >(wire_type, value, buf, ctx)
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, "nanos");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.seconds != 0i64 {
            ::prost::encoding::int64::encoded_len(1u32, &self.seconds)
        } else {
            0
        } + if self.nanos != 0i32 {
            ::prost::encoding::int32::encoded_len(2u32, &self.nanos)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.seconds = 0i64;
        self.nanos = 0i32;
    }
}
impl ::core::default::Default for Timestamp {
    fn default() -> Self {
        Timestamp {
            seconds: 0i64,
            nanos: 0i32,
        }
    }
}
impl ::core::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Timestamp");
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.seconds)
            };
            builder.field("seconds", &wrapper)
        };
        let builder = {
            let wrapper = {
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.nanos)
            };
            builder.field("nanos", &wrapper)
        };
        builder.finish()
    }
}
pub mod compiler {
    /// The version number of protocol compiler.
    pub struct Version {
        #[prost(int32, optional, tag = "1")]
        pub major: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub minor: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub patch: ::core::option::Option<i32>,
        /// A suffix for alpha, beta or rc release, e.g., "alpha-1", "rc2". It should
        /// be empty for mainline stable releases.
        #[prost(string, optional, tag = "4")]
        pub suffix: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Version {
        #[inline]
        fn clone(&self) -> Version {
            Version {
                major: ::core::clone::Clone::clone(&self.major),
                minor: ::core::clone::Clone::clone(&self.minor),
                patch: ::core::clone::Clone::clone(&self.patch),
                suffix: ::core::clone::Clone::clone(&self.suffix),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Version {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Version {
        #[inline]
        fn eq(&self, other: &Version) -> bool {
            self.major == other.major
                && self.minor == other.minor
                && self.patch == other.patch
                && self.suffix == other.suffix
        }
    }
    impl ::prost::Message for Version {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(1u32, &self.major, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(2u32, &self.minor, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Int32,
                _,
                _,
            >(3u32, &self.patch, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(4u32, &self.suffix, buf);
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "Version";
            match tag {
                1u32 => {
                    let mut value = &mut self.major;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "major");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.minor;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "minor");
                        error
                    })
                }
                3u32 => {
                    let mut value = &mut self.patch;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Int32,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "patch");
                        error
                    })
                }
                4u32 => {
                    let mut value = &mut self.suffix;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "suffix");
                        error
                    })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.major.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(1u32, value)
            }) + self.minor.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(2u32, value)
            }) + self.patch.as_ref().map_or(0, |value| {
                ::prost::encoding::int32::encoded_len(3u32, value)
            }) + self.suffix.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(4u32, value)
            })
        }
        fn clear(&mut self) {
            self.major = ::core::option::Option::None;
            self.minor = ::core::option::Option::None;
            self.patch = ::core::option::Option::None;
            self.suffix = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for Version {
        fn default() -> Self {
            Version {
                major: ::core::option::Option::None,
                minor: ::core::option::Option::None,
                patch: ::core::option::Option::None,
                suffix: ::core::option::Option::None,
            }
        }
    }
    impl ::core::fmt::Debug for Version {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Version");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.major)
                };
                builder.field("major", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.minor)
                };
                builder.field("minor", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.patch)
                };
                builder.field("patch", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.suffix)
                };
                builder.field("suffix", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl Version {
        ///Returns the value of `major`, or the default value if `major` is unset.
        pub fn major(&self) -> i32 {
            match self.major {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `minor`, or the default value if `minor` is unset.
        pub fn minor(&self) -> i32 {
            match self.minor {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `patch`, or the default value if `patch` is unset.
        pub fn patch(&self) -> i32 {
            match self.patch {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `suffix`, or the default value if `suffix` is unset.
        pub fn suffix(&self) -> &str {
            match self.suffix {
                ::core::option::Option::Some(ref val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
    }
    /// An encoded CodeGeneratorRequest is written to the plugin's stdin.
    pub struct CodeGeneratorRequest {
        /// The .proto files that were explicitly listed on the command-line.  The
        /// code generator should generate code only for these files.  Each file's
        /// descriptor will be included in proto_file, below.
        #[prost(string, repeated, tag = "1")]
        pub file_to_generate: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The generator parameter passed on the command-line.
        #[prost(string, optional, tag = "2")]
        pub parameter: ::core::option::Option<::prost::alloc::string::String>,
        /// FileDescriptorProtos for all files in files_to_generate and everything
        /// they import.  The files will appear in topological order, so each file
        /// appears before any file that imports it.
        ///
        /// protoc guarantees that all proto_files will be written after
        /// the fields above, even though this is not technically guaranteed by the
        /// protobuf wire format.  This theoretically could allow a plugin to stream
        /// in the FileDescriptorProtos and handle them one by one rather than read
        /// the entire set into memory at once.  However, as of this writing, this
        /// is not similarly optimized on protoc's end -- it will store all fields in
        /// memory at once before sending them to the plugin.
        ///
        /// Type names of fields and extensions in the FileDescriptorProto are always
        /// fully qualified.
        #[prost(message, repeated, tag = "15")]
        pub proto_file: ::prost::alloc::vec::Vec<super::FileDescriptorProto>,
        /// The version number of protocol compiler.
        #[prost(message, optional, tag = "3")]
        pub compiler_version: ::core::option::Option<Version>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CodeGeneratorRequest {
        #[inline]
        fn clone(&self) -> CodeGeneratorRequest {
            CodeGeneratorRequest {
                file_to_generate: ::core::clone::Clone::clone(&self.file_to_generate),
                parameter: ::core::clone::Clone::clone(&self.parameter),
                proto_file: ::core::clone::Clone::clone(&self.proto_file),
                compiler_version: ::core::clone::Clone::clone(&self.compiler_version),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CodeGeneratorRequest {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CodeGeneratorRequest {
        #[inline]
        fn eq(&self, other: &CodeGeneratorRequest) -> bool {
            self.file_to_generate == other.file_to_generate
                && self.parameter == other.parameter
                && self.proto_file == other.proto_file
                && self.compiler_version == other.compiler_version
        }
    }
    impl ::prost::Message for CodeGeneratorRequest {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.file_to_generate, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(2u32, &self.parameter, buf);
            if let Some(ref msg) = self.compiler_version {
                ::prost::encoding::message::encode(3u32, msg, buf);
            }
            for msg in &self.proto_file {
                ::prost::encoding::message::encode(15u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "CodeGeneratorRequest";
            match tag {
                1u32 => {
                    let mut value = &mut self.file_to_generate;
                    <::prost::encoding_traits::Repeated as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "file_to_generate");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.parameter;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "parameter");
                        error
                    })
                }
                3u32 => {
                    let mut value = &mut self.compiler_version;
                    ::prost::encoding::message::merge(
                        wire_type,
                        value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "compiler_version");
                        error
                    })
                }
                15u32 => {
                    let mut value = &mut self.proto_file;
                    ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, "proto_file");
                            error
                        },
                    )
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::string::encoded_len_repeated(1u32, &self.file_to_generate)
                + self.parameter.as_ref().map_or(0, |value| {
                    ::prost::encoding::string::encoded_len(2u32, value)
                })
                + self
                    .compiler_version
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(3u32, msg))
                + ::prost::encoding::message::encoded_len_repeated(15u32, &self.proto_file)
        }
        fn clear(&mut self) {
            self.file_to_generate.clear();
            self.parameter = ::core::option::Option::None;
            self.compiler_version = ::core::option::Option::None;
            self.proto_file.clear();
        }
    }
    impl ::core::default::Default for CodeGeneratorRequest {
        fn default() -> Self {
            CodeGeneratorRequest {
                file_to_generate: ::prost::alloc::vec::Vec::new(),
                parameter: ::core::option::Option::None,
                compiler_version: ::core::default::Default::default(),
                proto_file: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for CodeGeneratorRequest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("CodeGeneratorRequest");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.file_to_generate)
                };
                builder.field("file_to_generate", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.parameter)
                };
                builder.field("parameter", &wrapper)
            };
            let builder = {
                let wrapper = &self.proto_file;
                builder.field("proto_file", &wrapper)
            };
            let builder = {
                let wrapper = &self.compiler_version;
                builder.field("compiler_version", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl CodeGeneratorRequest {
        ///Returns the value of `parameter`, or the default value if `parameter` is unset.
        pub fn parameter(&self) -> &str {
            match self.parameter {
                ::core::option::Option::Some(ref val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
    }
    /// The plugin writes an encoded CodeGeneratorResponse to stdout.
    pub struct CodeGeneratorResponse {
        /// Error message.  If non-empty, code generation failed.  The plugin process
        /// should exit with status code zero even if it reports an error in this way.
        ///
        /// This should be used to indicate errors in .proto files which prevent the
        /// code generator from generating correct code.  Errors which indicate a
        /// problem in protoc itself -- such as the input CodeGeneratorRequest being
        /// unparseable -- should be reported by writing a message to stderr and
        /// exiting with a non-zero status code.
        #[prost(string, optional, tag = "1")]
        pub error: ::core::option::Option<::prost::alloc::string::String>,
        /// A bitmask of supported features that the code generator supports.
        /// This is a bitwise "or" of values from the Feature enum.
        #[prost(uint64, optional, tag = "2")]
        pub supported_features: ::core::option::Option<u64>,
        #[prost(message, repeated, tag = "15")]
        pub file: ::prost::alloc::vec::Vec<code_generator_response::File>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CodeGeneratorResponse {
        #[inline]
        fn clone(&self) -> CodeGeneratorResponse {
            CodeGeneratorResponse {
                error: ::core::clone::Clone::clone(&self.error),
                supported_features: ::core::clone::Clone::clone(&self.supported_features),
                file: ::core::clone::Clone::clone(&self.file),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CodeGeneratorResponse {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CodeGeneratorResponse {
        #[inline]
        fn eq(&self, other: &CodeGeneratorResponse) -> bool {
            self.error == other.error
                && self.supported_features == other.supported_features
                && self.file == other.file
        }
    }
    impl ::prost::Message for CodeGeneratorResponse {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::ProtobufString,
                _,
                _,
            >(1u32, &self.error, buf);
            <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                ::prost::encoding_traits::Uint64,
                _,
                _,
            >(2u32, &self.supported_features, buf);
            for msg in &self.file {
                ::prost::encoding::message::encode(15u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "CodeGeneratorResponse";
            match tag {
                1u32 => {
                    let mut value = &mut self.error;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::ProtobufString,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "error");
                        error
                    })
                }
                2u32 => {
                    let mut value = &mut self.supported_features;
                    <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::merge::<
                        ::prost::encoding_traits::Uint64,
                        _,
                        _,
                    >(wire_type, value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, "supported_features");
                        error
                    })
                }
                15u32 => {
                    let mut value = &mut self.file;
                    ::prost::encoding::message::merge_repeated(wire_type, value, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, "file");
                            error
                        },
                    )
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.error.as_ref().map_or(0, |value| {
                ::prost::encoding::string::encoded_len(1u32, value)
            }) + self.supported_features.as_ref().map_or(0, |value| {
                ::prost::encoding::uint64::encoded_len(2u32, value)
            }) + ::prost::encoding::message::encoded_len_repeated(15u32, &self.file)
        }
        fn clear(&mut self) {
            self.error = ::core::option::Option::None;
            self.supported_features = ::core::option::Option::None;
            self.file.clear();
        }
    }
    impl ::core::default::Default for CodeGeneratorResponse {
        fn default() -> Self {
            CodeGeneratorResponse {
                error: ::core::option::Option::None,
                supported_features: ::core::option::Option::None,
                file: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for CodeGeneratorResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("CodeGeneratorResponse");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.error)
                };
                builder.field("error", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<u64>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.supported_features)
                };
                builder.field("supported_features", &wrapper)
            };
            let builder = {
                let wrapper = &self.file;
                builder.field("file", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl CodeGeneratorResponse {
        ///Returns the value of `error`, or the default value if `error` is unset.
        pub fn error(&self) -> &str {
            match self.error {
                ::core::option::Option::Some(ref val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
        ///Returns the value of `supported_features`, or the default value if `supported_features` is unset.
        pub fn supported_features(&self) -> u64 {
            match self.supported_features {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0u64,
            }
        }
    }
    /// Nested message and enum types in `CodeGeneratorResponse`.
    pub mod code_generator_response {
        /// Represents a single generated file.
        pub struct File {
            /// The file name, relative to the output directory.  The name must not
            /// contain "." or ".." components and must be relative, not be absolute (so,
            /// the file cannot lie outside the output directory).  "/" must be used as
            /// the path separator, not "".
            ///
            /// If the name is omitted, the content will be appended to the previous
            /// file.  This allows the generator to break large files into small chunks,
            /// and allows the generated text to be streamed back to protoc so that large
            /// files need not reside completely in memory at one time.  Note that as of
            /// this writing protoc does not optimize for this -- it will read the entire
            /// CodeGeneratorResponse before writing files to disk.
            #[prost(string, optional, tag = "1")]
            pub name: ::core::option::Option<::prost::alloc::string::String>,
            /// If non-empty, indicates that the named file should already exist, and the
            /// content here is to be inserted into that file at a defined insertion
            /// point.  This feature allows a code generator to extend the output
            /// produced by another code generator.  The original generator may provide
            /// insertion points by placing special annotations in the file that look
            /// like:
            /// @@protoc_insertion_point(NAME)
            /// The annotation can have arbitrary text before and after it on the line,
            /// which allows it to be placed in a comment.  NAME should be replaced with
            /// an identifier naming the point -- this is what other generators will use
            /// as the insertion_point.  Code inserted at this point will be placed
            /// immediately above the line containing the insertion point (thus multiple
            /// insertions to the same point will come out in the order they were added).
            /// The double-@ is intended to make it unlikely that the generated code
            /// could contain things that look like insertion points by accident.
            ///
            /// For example, the C++ code generator places the following line in the
            /// .pb.h files that it generates:
            /// // @@protoc_insertion_point(namespace_scope)
            /// This line appears within the scope of the file's package namespace, but
            /// outside of any particular class.  Another plugin can then specify the
            /// insertion_point "namespace_scope" to generate additional classes or
            /// other declarations that should be placed in this scope.
            ///
            /// Note that if the line containing the insertion point begins with
            /// whitespace, the same whitespace will be added to every line of the
            /// inserted text.  This is useful for languages like Python, where
            /// indentation matters.  In these languages, the insertion point comment
            /// should be indented the same amount as any inserted code will need to be
            /// in order to work correctly in that context.
            ///
            /// The code generator that generates the initial file and the one which
            /// inserts into it must both run as part of a single invocation of protoc.
            /// Code generators are executed in the order in which they appear on the
            /// command line.
            ///
            /// If |insertion_point| is present, |name| must also be present.
            #[prost(string, optional, tag = "2")]
            pub insertion_point: ::core::option::Option<::prost::alloc::string::String>,
            /// The file contents.
            #[prost(string, optional, tag = "15")]
            pub content: ::core::option::Option<::prost::alloc::string::String>,
            /// Information describing the file content being inserted. If an insertion
            /// point is used, this information will be appropriately offset and inserted
            /// into the code generation metadata for the generated files.
            #[prost(message, optional, tag = "16")]
            pub generated_code_info: ::core::option::Option<super::super::GeneratedCodeInfo>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for File {
            #[inline]
            fn clone(&self) -> File {
                File {
                    name: ::core::clone::Clone::clone(&self.name),
                    insertion_point: ::core::clone::Clone::clone(&self.insertion_point),
                    content: ::core::clone::Clone::clone(&self.content),
                    generated_code_info: ::core::clone::Clone::clone(&self.generated_code_info),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for File {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for File {
            #[inline]
            fn eq(&self, other: &File) -> bool {
                self.name == other.name
                    && self.insertion_point == other.insertion_point
                    && self.content == other.content
                    && self.generated_code_info == other.generated_code_info
            }
        }
        impl ::prost::Message for File {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(1u32, &self.name, buf);
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(2u32, &self.insertion_point, buf);
                <::prost::encoding_traits::Optional as ::prost::encoding_traits::Mode>::encode::<
                    ::prost::encoding_traits::ProtobufString,
                    _,
                    _,
                >(15u32, &self.content, buf);
                if let Some(ref msg) = self.generated_code_info {
                    ::prost::encoding::message::encode(16u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "File";
                match tag {
                    1u32 => {
                        let mut value = &mut self.name;
                        < :: prost :: encoding_traits :: Optional as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufString , _ , _ > (wire_type , value , buf , ctx) . map_err (| mut error | { error . push (STRUCT_NAME , "name") ; error })
                    }
                    2u32 => {
                        let mut value = &mut self.insertion_point;
                        < :: prost :: encoding_traits :: Optional as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufString , _ , _ > (wire_type , value , buf , ctx) . map_err (| mut error | { error . push (STRUCT_NAME , "insertion_point") ; error })
                    }
                    15u32 => {
                        let mut value = &mut self.content;
                        < :: prost :: encoding_traits :: Optional as :: prost :: encoding_traits :: Mode > :: merge :: < :: prost :: encoding_traits :: ProtobufString , _ , _ > (wire_type , value , buf , ctx) . map_err (| mut error | { error . push (STRUCT_NAME , "content") ; error })
                    }
                    16u32 => {
                        let mut value = &mut self.generated_code_info;
                        ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "generated_code_info");
                            error
                        })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.name.as_ref().map_or(0, |value| {
                    ::prost::encoding::string::encoded_len(1u32, value)
                }) + self.insertion_point.as_ref().map_or(0, |value| {
                    ::prost::encoding::string::encoded_len(2u32, value)
                }) + self.content.as_ref().map_or(0, |value| {
                    ::prost::encoding::string::encoded_len(15u32, value)
                }) + self
                    .generated_code_info
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(16u32, msg))
            }
            fn clear(&mut self) {
                self.name = ::core::option::Option::None;
                self.insertion_point = ::core::option::Option::None;
                self.content = ::core::option::Option::None;
                self.generated_code_info = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for File {
            fn default() -> Self {
                File {
                    name: ::core::option::Option::None,
                    insertion_point: ::core::option::Option::None,
                    content: ::core::option::Option::None,
                    generated_code_info: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for File {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("File");
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(
                            &'a ::core::option::Option<::prost::alloc::string::String>,
                        );
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.name)
                    };
                    builder.field("name", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(
                            &'a ::core::option::Option<::prost::alloc::string::String>,
                        );
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.insertion_point)
                    };
                    builder.field("insertion_point", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(
                            &'a ::core::option::Option<::prost::alloc::string::String>,
                        );
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.content)
                    };
                    builder.field("content", &wrapper)
                };
                let builder = {
                    let wrapper = &self.generated_code_info;
                    builder.field("generated_code_info", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl File {
            ///Returns the value of `name`, or the default value if `name` is unset.
            pub fn name(&self) -> &str {
                match self.name {
                    ::core::option::Option::Some(ref val) => &val[..],
                    ::core::option::Option::None => "",
                }
            }
            ///Returns the value of `insertion_point`, or the default value if `insertion_point` is unset.
            pub fn insertion_point(&self) -> &str {
                match self.insertion_point {
                    ::core::option::Option::Some(ref val) => &val[..],
                    ::core::option::Option::None => "",
                }
            }
            ///Returns the value of `content`, or the default value if `content` is unset.
            pub fn content(&self) -> &str {
                match self.content {
                    ::core::option::Option::Some(ref val) => &val[..],
                    ::core::option::Option::None => "",
                }
            }
        }
        /// Sync with code_generator.h.
        #[repr(i32)]
        pub enum Feature {
            None = 0,
            Proto3Optional = 1,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Feature {
            #[inline]
            fn clone(&self) -> Feature {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Feature {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Feature {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Feature::None => ::core::fmt::Formatter::write_str(f, "None"),
                    Feature::Proto3Optional => {
                        ::core::fmt::Formatter::write_str(f, "Proto3Optional")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Feature {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Feature {
            #[inline]
            fn eq(&self, other: &Feature) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Feature {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Feature {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Feature {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Feature {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Feature,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Feature {
            #[inline]
            fn cmp(&self, other: &Feature) -> ::core::cmp::Ordering {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
            }
        }
        impl Feature {
            ///Returns `true` if `value` is a variant of `Feature`.
            pub fn is_valid(value: i32) -> bool {
                match value {
                    0 => true,
                    1 => true,
                    _ => false,
                }
            }
            ///Converts an `i32` to a `Feature`, or `None` if `value` is not a valid variant.
            pub fn from_i32(value: i32) -> ::core::option::Option<Feature> {
                match value {
                    0 => ::core::option::Option::Some(Feature::None),
                    1 => ::core::option::Option::Some(Feature::Proto3Optional),
                    _ => ::core::option::Option::None,
                }
            }
        }
        impl ::core::default::Default for Feature {
            fn default() -> Feature {
                Feature::None
            }
        }
        impl ::core::convert::From<Feature> for i32 {
            fn from(value: Feature) -> i32 {
                value as i32
            }
        }
        impl Feature {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Feature::None => "FEATURE_NONE",
                    Feature::Proto3Optional => "FEATURE_PROTO3_OPTIONAL",
                }
            }
        }
    }
}
mod datetime {
    //! A date/time type which exists primarily to convert [`Timestamp`]s into an RFC 3339 formatted
    //! string.
    use core::fmt;
    use crate::Duration;
    use crate::Timestamp;
    /// A point in time, represented as a date and time in the UTC timezone.
    pub(crate) struct DateTime {
        /// The year.
        pub(crate) year: i64,
        /// The month of the year, from 1 to 12, inclusive.
        pub(crate) month: u8,
        /// The day of the month, from 1 to 31, inclusive.
        pub(crate) day: u8,
        /// The hour of the day, from 0 to 23, inclusive.
        pub(crate) hour: u8,
        /// The minute of the hour, from 0 to 59, inclusive.
        pub(crate) minute: u8,
        /// The second of the minute, from 0 to 59, inclusive.
        pub(crate) second: u8,
        /// The nanoseconds, from 0 to 999_999_999, inclusive.
        pub(crate) nanos: u32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DateTime {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &["year", "month", "day", "hour", "minute", "second", "nanos"];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.year,
                &&self.month,
                &&self.day,
                &&self.hour,
                &&self.minute,
                &&self.second,
                &&self.nanos,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "DateTime", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for DateTime {
        #[inline]
        fn default() -> DateTime {
            DateTime {
                year: ::core::default::Default::default(),
                month: ::core::default::Default::default(),
                day: ::core::default::Default::default(),
                hour: ::core::default::Default::default(),
                minute: ::core::default::Default::default(),
                second: ::core::default::Default::default(),
                nanos: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DateTime {
        #[inline]
        fn clone(&self) -> DateTime {
            let _: ::core::clone::AssertParamIsClone<i64>;
            let _: ::core::clone::AssertParamIsClone<u8>;
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DateTime {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DateTime {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DateTime {
        #[inline]
        fn eq(&self, other: &DateTime) -> bool {
            self.year == other.year
                && self.month == other.month
                && self.day == other.day
                && self.hour == other.hour
                && self.minute == other.minute
                && self.second == other.second
                && self.nanos == other.nanos
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for DateTime {}
    #[automatically_derived]
    impl ::core::cmp::Eq for DateTime {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i64>;
            let _: ::core::cmp::AssertParamIsEq<u8>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DateTime {
        #[inline]
        fn partial_cmp(&self, other: &DateTime) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.year, &other.year) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.month, &other.month) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(&self.day, &other.day) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(
                                        &self.hour,
                                        &other.hour,
                                    ) {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => match ::core::cmp::PartialOrd::partial_cmp(
                                            &self.minute,
                                            &other.minute,
                                        ) {
                                            ::core::option::Option::Some(
                                                ::core::cmp::Ordering::Equal,
                                            ) => match ::core::cmp::PartialOrd::partial_cmp(
                                                &self.second,
                                                &other.second,
                                            ) {
                                                ::core::option::Option::Some(
                                                    ::core::cmp::Ordering::Equal,
                                                ) => ::core::cmp::PartialOrd::partial_cmp(
                                                    &self.nanos,
                                                    &other.nanos,
                                                ),
                                                cmp => cmp,
                                            },
                                            cmp => cmp,
                                        },
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DateTime {
        #[inline]
        fn cmp(&self, other: &DateTime) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.year, &other.year) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(&self.month, &other.month) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&self.day, &other.day) {
                                ::core::cmp::Ordering::Equal => {
                                    match ::core::cmp::Ord::cmp(&self.hour, &other.hour) {
                                        ::core::cmp::Ordering::Equal => {
                                            match ::core::cmp::Ord::cmp(&self.minute, &other.minute)
                                            {
                                                ::core::cmp::Ordering::Equal => {
                                                    match ::core::cmp::Ord::cmp(
                                                        &self.second,
                                                        &other.second,
                                                    ) {
                                                        ::core::cmp::Ordering::Equal => {
                                                            ::core::cmp::Ord::cmp(
                                                                &self.nanos,
                                                                &other.nanos,
                                                            )
                                                        }
                                                        cmp => cmp,
                                                    }
                                                }
                                                cmp => cmp,
                                            }
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    impl DateTime {
        /// The minimum representable [`Timestamp`] as a `DateTime`.
        pub(crate) const MIN: DateTime = DateTime {
            year: -292_277_022_657,
            month: 1,
            day: 27,
            hour: 8,
            minute: 29,
            second: 52,
            nanos: 0,
        };
        /// The maximum representable [`Timestamp`] as a `DateTime`.
        pub(crate) const MAX: DateTime = DateTime {
            year: 292_277_026_596,
            month: 12,
            day: 4,
            hour: 15,
            minute: 30,
            second: 7,
            nanos: 999_999_999,
        };
        /// Returns `true` if the `DateTime` is a valid calendar date.
        pub(crate) fn is_valid(&self) -> bool {
            self >= &DateTime::MIN
                && self <= &DateTime::MAX
                && self.month > 0
                && self.month <= 12
                && self.day > 0
                && self.day <= days_in_month(self.year, self.month)
                && self.hour < 24
                && self.minute < 60
                && self.second < 60
                && self.nanos < 1_000_000_000
        }
    }
    impl fmt::Display for DateTime {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.year > 9999 {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["+"],
                    &[::core::fmt::ArgumentV1::new_display(&self.year)],
                ))?;
            } else if self.year < 0 {
                f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&self.year)],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(5usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ))?;
            } else {
                f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&self.year)],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(4usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ))?;
            };
            f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                &["-", "-", "T", ":", ":"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.month),
                    ::core::fmt::ArgumentV1::new_display(&self.day),
                    ::core::fmt::ArgumentV1::new_display(&self.hour),
                    ::core::fmt::ArgumentV1::new_display(&self.minute),
                    ::core::fmt::ArgumentV1::new_display(&self.second),
                ],
                &[
                    ::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(2usize),
                        },
                    },
                    ::core::fmt::rt::v1::Argument {
                        position: 1usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(2usize),
                        },
                    },
                    ::core::fmt::rt::v1::Argument {
                        position: 2usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(2usize),
                        },
                    },
                    ::core::fmt::rt::v1::Argument {
                        position: 3usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(2usize),
                        },
                    },
                    ::core::fmt::rt::v1::Argument {
                        position: 4usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(2usize),
                        },
                    },
                ],
                unsafe { ::core::fmt::UnsafeArg::new() },
            ))?;
            let nanos = self.nanos;
            if nanos == 0 {
                f.write_fmt(::core::fmt::Arguments::new_v1(&["Z"], &[]))
            } else if nanos % 1_000_000 == 0 {
                f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                    &[".", "Z"],
                    &[::core::fmt::ArgumentV1::new_display(&(nanos / 1_000_000))],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(3usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ))
            } else if nanos % 1_000 == 0 {
                f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                    &[".", "Z"],
                    &[::core::fmt::ArgumentV1::new_display(&(nanos / 1_000))],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ))
            } else {
                f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                    &[".", "Z"],
                    &[::core::fmt::ArgumentV1::new_display(&nanos)],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(9usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ))
            }
        }
    }
    impl From<Timestamp> for DateTime {
        /// musl's [`__secs_to_tm`][1] converted to Rust via [c2rust][2] and then cleaned up by hand.
        ///
        /// All existing `strftime`-like APIs in Rust are unable to handle the full range of timestamps
        /// representable by `Timestamp`, including `strftime` itself, since tm.tm_year is an int.
        ///
        /// [1]: http://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c
        /// [2]: https://c2rust.com/
        fn from(mut timestamp: Timestamp) -> DateTime {
            timestamp.normalize();
            let t = timestamp.seconds;
            let nanos = timestamp.nanos;
            const LEAPOCH: i64 = 946_684_800 + 86400 * (31 + 29);
            const DAYS_PER_400Y: i32 = 365 * 400 + 97;
            const DAYS_PER_100Y: i32 = 365 * 100 + 24;
            const DAYS_PER_4Y: i32 = 365 * 4 + 1;
            const DAYS_IN_MONTH: [u8; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
            let mut days: i64 = (t / 86_400) - (LEAPOCH / 86_400);
            let mut remsecs: i32 = (t % 86_400) as i32;
            if remsecs < 0i32 {
                remsecs += 86_400;
                days -= 1
            }
            let mut qc_cycles: i32 = (days / i64::from(DAYS_PER_400Y)) as i32;
            let mut remdays: i32 = (days % i64::from(DAYS_PER_400Y)) as i32;
            if remdays < 0 {
                remdays += DAYS_PER_400Y;
                qc_cycles -= 1;
            }
            let mut c_cycles: i32 = remdays / DAYS_PER_100Y;
            if c_cycles == 4 {
                c_cycles -= 1;
            }
            remdays -= c_cycles * DAYS_PER_100Y;
            let mut q_cycles: i32 = remdays / DAYS_PER_4Y;
            if q_cycles == 25 {
                q_cycles -= 1;
            }
            remdays -= q_cycles * DAYS_PER_4Y;
            let mut remyears: i32 = remdays / 365;
            if remyears == 4 {
                remyears -= 1;
            }
            remdays -= remyears * 365;
            let mut years: i64 = i64::from(remyears)
                + 4 * i64::from(q_cycles)
                + 100 * i64::from(c_cycles)
                + 400 * i64::from(qc_cycles);
            let mut months: i32 = 0;
            while i32::from(DAYS_IN_MONTH[months as usize]) <= remdays {
                remdays -= i32::from(DAYS_IN_MONTH[months as usize]);
                months += 1
            }
            if months >= 10 {
                months -= 12;
                years += 1;
            }
            let date_time = DateTime {
                year: years + 2000,
                month: (months + 3) as u8,
                day: (remdays + 1) as u8,
                hour: (remsecs / 3600) as u8,
                minute: (remsecs / 60 % 60) as u8,
                second: (remsecs % 60) as u8,
                nanos: nanos as u32,
            };
            if true {
                if !date_time.is_valid() {
                    ::core::panicking::panic("assertion failed: date_time.is_valid()")
                };
            };
            date_time
        }
    }
    /// Returns the number of days in the month.
    fn days_in_month(year: i64, month: u8) -> u8 {
        const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let (_, is_leap) = year_to_seconds(year);
        DAYS_IN_MONTH[usize::from(month - 1)] + u8::from(is_leap && month == 2)
    }
    /// Parses a date in RFC 3339 format from ASCII string `b`, returning the year, month, day, and
    /// remaining input.
    ///
    /// The date is not validated according to a calendar.
    fn parse_date(s: &str) -> Option<(i64, u8, u8, &str)> {
        if true {
            if !s.is_ascii() {
                ::core::panicking::panic("assertion failed: s.is_ascii()")
            };
        };
        {
            if !(s.len() >= 10) {
                return None;
            }
        };
        let (year, s) = match s.as_bytes()[0] {
            b'+' => {
                let (digits, s) = parse_digits(&s[1..]);
                {
                    if !(digits.len() >= 5) {
                        return None;
                    }
                };
                let date: i64 = digits.parse().ok()?;
                (date, s)
            }
            b'-' => {
                let (digits, s) = parse_digits(&s[1..]);
                {
                    if !(digits.len() >= 4) {
                        return None;
                    }
                };
                let date: i64 = digits.parse().ok()?;
                (-date, s)
            }
            _ => {
                let (n1, s) = parse_two_digit_numeric(s)?;
                let (n2, s) = parse_two_digit_numeric(s)?;
                (i64::from(n1) * 100 + i64::from(n2), s)
            }
        };
        let s = parse_char(s, b'-')?;
        let (month, s) = parse_two_digit_numeric(s)?;
        let s = parse_char(s, b'-')?;
        let (day, s) = parse_two_digit_numeric(s)?;
        Some((year, month, day, s))
    }
    /// Parses a time in RFC 3339 format from ASCII string `s`, returning the hour, minute, second, and
    /// nanos.
    ///
    /// The date is not validated according to a calendar.
    fn parse_time(s: &str) -> Option<(u8, u8, u8, u32, &str)> {
        if true {
            if !s.is_ascii() {
                ::core::panicking::panic("assertion failed: s.is_ascii()")
            };
        };
        let (hour, s) = parse_two_digit_numeric(s)?;
        let s = parse_char(s, b':')?;
        let (minute, s) = parse_two_digit_numeric(s)?;
        let s = parse_char(s, b':')?;
        let (second, s) = parse_two_digit_numeric(s)?;
        let (nanos, s) = parse_nanos(s)?;
        Some((hour, minute, second, nanos, s))
    }
    /// Parses an optional nanosecond time from ASCII string `s`, returning the nanos and remaining
    /// string.
    fn parse_nanos(s: &str) -> Option<(u32, &str)> {
        if true {
            if !s.is_ascii() {
                ::core::panicking::panic("assertion failed: s.is_ascii()")
            };
        };
        let (nanos, s) = if let Some(s) = parse_char(s, b'.') {
            let (digits, s) = parse_digits(s);
            {
                if !(digits.len() <= 9) {
                    return None;
                }
            };
            let nanos = 10u32.pow(9 - digits.len() as u32) * digits.parse::<u32>().ok()?;
            (nanos, s)
        } else {
            (0, s)
        };
        Some((nanos, s))
    }
    /// Parses a timezone offset in RFC 3339 format from ASCII string `s`, returning the offset hour,
    /// offset minute, and remaining input.
    fn parse_offset(s: &str) -> Option<(i8, i8, &str)> {
        if true {
            if !s.is_ascii() {
                ::core::panicking::panic("assertion failed: s.is_ascii()")
            };
        };
        if s.is_empty() {
            return Some((0, 0, s));
        }
        let s = parse_char(s, b' ').unwrap_or(s);
        if let Some(s) = parse_char_ignore_case(s, b'Z') {
            Some((0, 0, s))
        } else {
            let (is_positive, s) = if let Some(s) = parse_char(s, b'+') {
                (true, s)
            } else if let Some(s) = parse_char(s, b'-') {
                (false, s)
            } else {
                return None;
            };
            let (hour, s) = parse_two_digit_numeric(s)?;
            let (minute, s) = if s.is_empty() {
                (0, s)
            } else {
                let s = parse_char(s, b':').unwrap_or(s);
                let (minute, s) = parse_two_digit_numeric(s)?;
                (minute, s)
            };
            {
                if !(is_positive || hour > 0 || minute > 0) {
                    return None;
                }
            };
            {
                if !(hour < 24 && minute < 60) {
                    return None;
                }
            };
            let hour = hour as i8;
            let minute = minute as i8;
            if is_positive {
                Some((hour, minute, s))
            } else {
                Some((-hour, -minute, s))
            }
        }
    }
    /// Parses a two-digit base-10 number from ASCII string `s`, returning the number and the remaining
    /// string.
    fn parse_two_digit_numeric(s: &str) -> Option<(u8, &str)> {
        if true {
            if !s.is_ascii() {
                ::core::panicking::panic("assertion failed: s.is_ascii()")
            };
        };
        let (digits, s) = s.split_at(2);
        Some((digits.parse().ok()?, s))
    }
    /// Splits ASCII string `s` at the first occurrence of a non-digit character.
    fn parse_digits(s: &str) -> (&str, &str) {
        if true {
            if !s.is_ascii() {
                ::core::panicking::panic("assertion failed: s.is_ascii()")
            };
        };
        let idx = s
            .as_bytes()
            .iter()
            .position(|c| !c.is_ascii_digit())
            .unwrap_or(s.len());
        s.split_at(idx)
    }
    /// Attempts to parse ASCII character `c` from ASCII string `s`, returning the remaining string. If
    /// the character can not be parsed, returns `None`.
    fn parse_char(s: &str, c: u8) -> Option<&str> {
        if true {
            if !s.is_ascii() {
                ::core::panicking::panic("assertion failed: s.is_ascii()")
            };
        };
        {
            if !(*s.as_bytes().get(0)? == c) {
                return None;
            }
        };
        Some(&s[1..])
    }
    /// Attempts to parse ASCII character `c` from ASCII string `s`, ignoring ASCII case, returning the
    /// remaining string. If the character can not be parsed, returns `None`.
    fn parse_char_ignore_case(s: &str, c: u8) -> Option<&str> {
        if true {
            if !s.is_ascii() {
                ::core::panicking::panic("assertion failed: s.is_ascii()")
            };
        };
        {
            if !s.as_bytes().get(0)?.eq_ignore_ascii_case(&c) {
                return None;
            }
        };
        Some(&s[1..])
    }
    /// Returns the offset in seconds from the Unix epoch of the date time.
    ///
    /// This is musl's [`__tm_to_secs`][1] converted to Rust via [c2rust[2] and then cleaned up by
    /// hand.
    ///
    /// [1]: https://git.musl-libc.org/cgit/musl/tree/src/time/__tm_to_secs.c
    /// [2]: https://c2rust.com/
    fn date_time_to_seconds(tm: &DateTime) -> i64 {
        let (start_of_year, is_leap) = year_to_seconds(tm.year);
        let seconds_within_year = month_to_seconds(tm.month, is_leap)
            + 86400 * u32::from(tm.day - 1)
            + 3600 * u32::from(tm.hour)
            + 60 * u32::from(tm.minute)
            + u32::from(tm.second);
        (start_of_year + i128::from(seconds_within_year)) as i64
    }
    /// Returns the number of seconds in the year prior to the start of the provided month.
    ///
    /// This is musl's [`__month_to_secs`][1] converted to Rust via c2rust and then cleaned up by hand.
    ///
    /// [1]: https://git.musl-libc.org/cgit/musl/tree/src/time/__month_to_secs.c
    fn month_to_seconds(month: u8, is_leap: bool) -> u32 {
        const SECS_THROUGH_MONTH: [u32; 12] = [
            0,
            31 * 86400,
            59 * 86400,
            90 * 86400,
            120 * 86400,
            151 * 86400,
            181 * 86400,
            212 * 86400,
            243 * 86400,
            273 * 86400,
            304 * 86400,
            334 * 86400,
        ];
        let t = SECS_THROUGH_MONTH[usize::from(month - 1)];
        if is_leap && month > 2 {
            t + 86400
        } else {
            t
        }
    }
    /// Returns the offset in seconds from the Unix epoch of the start of a year.
    ///
    /// musl's [`__year_to_secs`][1] converted to Rust via c2rust and then cleaned up by hand.
    ///
    /// Returns an i128 because the start of the earliest supported year underflows i64.
    ///
    /// [1]: https://git.musl-libc.org/cgit/musl/tree/src/time/__year_to_secs.c
    pub(crate) fn year_to_seconds(year: i64) -> (i128, bool) {
        let is_leap;
        let year = year - 1900;
        if year as u64 <= 138 {
            let mut leaps: i64 = (year - 68) >> 2;
            if (year - 68).trailing_zeros() >= 2 {
                leaps -= 1;
                is_leap = true;
            } else {
                is_leap = false;
            }
            return (
                i128::from(31_536_000 * (year - 70) + 86400 * leaps),
                is_leap,
            );
        }
        let centuries: i64;
        let mut leaps: i64;
        let mut cycles: i64 = (year - 100) / 400;
        let mut rem: i64 = (year - 100) % 400;
        if rem < 0 {
            cycles -= 1;
            rem += 400
        }
        if rem == 0 {
            is_leap = true;
            centuries = 0;
            leaps = 0;
        } else {
            if rem >= 200 {
                if rem >= 300 {
                    centuries = 3;
                    rem -= 300;
                } else {
                    centuries = 2;
                    rem -= 200;
                }
            } else if rem >= 100 {
                centuries = 1;
                rem -= 100;
            } else {
                centuries = 0;
            }
            if rem == 0 {
                is_leap = false;
                leaps = 0;
            } else {
                leaps = rem / 4;
                rem %= 4;
                is_leap = rem == 0;
            }
        }
        leaps += 97 * cycles + 24 * centuries - i64::from(is_leap);
        (
            i128::from((year - 100) * 31_536_000) + i128::from(leaps * 86400 + 946_684_800 + 86400),
            is_leap,
        )
    }
    /// Parses a timestamp in RFC 3339 format from `s`.
    pub(crate) fn parse_timestamp(s: &str) -> Option<Timestamp> {
        {
            if !s.is_ascii() {
                return None;
            }
        };
        let (year, month, day, s) = parse_date(s)?;
        if s.is_empty() {
            let date_time = DateTime {
                year,
                month,
                day,
                ..DateTime::default()
            };
            {
                if !date_time.is_valid() {
                    return None;
                }
            };
            return Some(Timestamp::from(date_time));
        }
        let s = parse_char_ignore_case(s, b'T').or_else(|| parse_char(s, b' '))?;
        let (hour, minute, mut second, nanos, s) = parse_time(s)?;
        let (offset_hour, offset_minute, s) = parse_offset(s)?;
        {
            if !s.is_empty() {
                return None;
            }
        };
        if second == 60 {
            second = 59;
        }
        let date_time = DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanos,
        };
        {
            if !date_time.is_valid() {
                return None;
            }
        };
        let Timestamp { seconds, nanos } = Timestamp::from(date_time);
        let seconds =
            seconds.checked_sub(i64::from(offset_hour) * 3600 + i64::from(offset_minute) * 60)?;
        Some(Timestamp { seconds, nanos })
    }
    /// Parse a duration in the [Protobuf JSON encoding spec format][1].
    ///
    /// [1]: https://developers.google.com/protocol-buffers/docs/proto3#json
    pub(crate) fn parse_duration(s: &str) -> Option<Duration> {
        {
            if !s.is_ascii() {
                return None;
            }
        };
        let (is_negative, s) = match parse_char(s, b'-') {
            Some(s) => (true, s),
            None => (false, s),
        };
        let (digits, s) = parse_digits(s);
        let seconds = digits.parse::<i64>().ok()?;
        let (nanos, s) = parse_nanos(s)?;
        let s = parse_char(s, b's')?;
        {
            if !s.is_empty() {
                return None;
            }
        };
        {
            if !(nanos < crate::NANOS_PER_SECOND as u32) {
                return None;
            }
        };
        let (seconds, nanos) = if is_negative {
            (-seconds, -(nanos as i32))
        } else {
            (seconds, nanos as i32)
        };
        Some(Duration {
            seconds,
            nanos: nanos as i32,
        })
    }
    impl From<DateTime> for Timestamp {
        fn from(date_time: DateTime) -> Timestamp {
            let seconds = date_time_to_seconds(&date_time);
            let nanos = date_time.nanos;
            Timestamp {
                seconds,
                nanos: nanos as i32,
            }
        }
    }
}
const NANOS_PER_SECOND: i32 = 1_000_000_000;
const NANOS_MAX: i32 = NANOS_PER_SECOND - 1;
impl Duration {
    /// Normalizes the duration to a canonical format.
    ///
    /// Based on [`google::protobuf::util::CreateNormalized`][1].
    ///
    /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L79-L100
    pub fn normalize(&mut self) {
        if self.nanos <= -NANOS_PER_SECOND || self.nanos >= NANOS_PER_SECOND {
            if let Some(seconds) = self
                .seconds
                .checked_add((self.nanos / NANOS_PER_SECOND) as i64)
            {
                self.seconds = seconds;
                self.nanos %= NANOS_PER_SECOND;
            } else if self.nanos < 0 {
                self.seconds = i64::MIN;
                self.nanos = -NANOS_MAX;
            } else {
                self.seconds = i64::MAX;
                self.nanos = NANOS_MAX;
            }
        }
        if self.seconds < 0 && self.nanos > 0 {
            if let Some(seconds) = self.seconds.checked_add(1) {
                self.seconds = seconds;
                self.nanos -= NANOS_PER_SECOND;
            } else {
                if true {
                    match (&self.seconds, &i64::MAX) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                };
                self.nanos = NANOS_MAX;
            }
        } else if self.seconds > 0 && self.nanos < 0 {
            if let Some(seconds) = self.seconds.checked_sub(1) {
                self.seconds = seconds;
                self.nanos += NANOS_PER_SECOND;
            } else {
                if true {
                    match (&self.seconds, &i64::MIN) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                };
                self.nanos = -NANOS_MAX;
            }
        }
    }
}
impl TryFrom<time::Duration> for Duration {
    type Error = DurationError;
    /// Converts a `std::time::Duration` to a `Duration`, failing if the duration is too large.
    fn try_from(duration: time::Duration) -> Result<Duration, DurationError> {
        let seconds = i64::try_from(duration.as_secs()).map_err(|_| DurationError::OutOfRange)?;
        let nanos = duration.subsec_nanos() as i32;
        let mut duration = Duration { seconds, nanos };
        duration.normalize();
        Ok(duration)
    }
}
impl TryFrom<Duration> for time::Duration {
    type Error = DurationError;
    /// Converts a `Duration` to a `std::time::Duration`, failing if the duration is negative.
    fn try_from(mut duration: Duration) -> Result<time::Duration, DurationError> {
        duration.normalize();
        if duration.seconds >= 0 {
            Ok(time::Duration::new(
                duration.seconds as u64,
                duration.nanos as u32,
            ))
        } else {
            Err(DurationError::NegativeDuration(time::Duration::new(
                (-duration.seconds) as u64,
                (-duration.nanos) as u32,
            )))
        }
    }
}
impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = self.clone();
        d.normalize();
        if self.seconds < 0 && self.nanos < 0 {
            f.write_fmt(::core::fmt::Arguments::new_v1(&["-"], &[]))?;
        }
        f.write_fmt(::core::fmt::Arguments::new_v1(
            &[""],
            &[::core::fmt::ArgumentV1::new_display(&d.seconds.abs())],
        ))?;
        let nanos = d.nanos.abs();
        if nanos == 0 {
            f.write_fmt(::core::fmt::Arguments::new_v1(&["s"], &[]))
        } else if nanos % 1_000_000 == 0 {
            f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                &[".", "s"],
                &[::core::fmt::ArgumentV1::new_display(&(nanos / 1_000_000))],
                &[::core::fmt::rt::v1::Argument {
                    position: 0usize,
                    format: ::core::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                        flags: 8u32,
                        precision: ::core::fmt::rt::v1::Count::Implied,
                        width: ::core::fmt::rt::v1::Count::Is(3usize),
                    },
                }],
                unsafe { ::core::fmt::UnsafeArg::new() },
            ))
        } else if nanos % 1_000 == 0 {
            f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                &[".", "s"],
                &[::core::fmt::ArgumentV1::new_display(&(nanos / 1_000))],
                &[::core::fmt::rt::v1::Argument {
                    position: 0usize,
                    format: ::core::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                        flags: 8u32,
                        precision: ::core::fmt::rt::v1::Count::Implied,
                        width: ::core::fmt::rt::v1::Count::Is(6usize),
                    },
                }],
                unsafe { ::core::fmt::UnsafeArg::new() },
            ))
        } else {
            f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                &[".", "s"],
                &[::core::fmt::ArgumentV1::new_display(&nanos)],
                &[::core::fmt::rt::v1::Argument {
                    position: 0usize,
                    format: ::core::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                        flags: 8u32,
                        precision: ::core::fmt::rt::v1::Count::Implied,
                        width: ::core::fmt::rt::v1::Count::Is(9usize),
                    },
                }],
                unsafe { ::core::fmt::UnsafeArg::new() },
            ))
        }
    }
}
/// A duration handling error.
#[non_exhaustive]
pub enum DurationError {
    /// Indicates failure to parse a [`Duration`] from a string.
    ///
    /// The [`Duration`] string format is specified in the [Protobuf JSON mapping specification][1].
    ///
    /// [1]: https://developers.google.com/protocol-buffers/docs/proto3#json
    ParseFailure,
    /// Indicates failure to convert a `prost_types::Duration` to a `std::time::Duration` because
    /// the duration is negative. The included `std::time::Duration` matches the magnitude of the
    /// original negative `prost_types::Duration`.
    NegativeDuration(time::Duration),
    /// Indicates failure to convert a `std::time::Duration` to a `prost_types::Duration`.
    ///
    /// Converting a `std::time::Duration` to a `prost_types::Duration` fails if the magnitude
    /// exceeds that representable by `prost_types::Duration`.
    OutOfRange,
}
#[automatically_derived]
impl ::core::fmt::Debug for DurationError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DurationError::ParseFailure => ::core::fmt::Formatter::write_str(f, "ParseFailure"),
            DurationError::NegativeDuration(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "NegativeDuration", &__self_0)
            }
            DurationError::OutOfRange => ::core::fmt::Formatter::write_str(f, "OutOfRange"),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for DurationError {}
#[automatically_derived]
impl ::core::cmp::PartialEq for DurationError {
    #[inline]
    fn eq(&self, other: &DurationError) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    DurationError::NegativeDuration(__self_0),
                    DurationError::NegativeDuration(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => true,
            }
    }
}
impl fmt::Display for DurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurationError::ParseFailure => f.write_fmt(::core::fmt::Arguments::new_v1(
                &["failed to parse duration"],
                &[],
            )),
            DurationError::NegativeDuration(duration) => {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["failed to convert negative duration: "],
                    &[::core::fmt::ArgumentV1::new_debug(&duration)],
                ))
            }
            DurationError::OutOfRange => f.write_fmt(::core::fmt::Arguments::new_v1(
                &["failed to convert duration out of range"],
                &[],
            )),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for DurationError {}
impl FromStr for Duration {
    type Err = DurationError;
    fn from_str(s: &str) -> Result<Duration, DurationError> {
        datetime::parse_duration(s).ok_or(DurationError::ParseFailure)
    }
}
impl Timestamp {
    /// Normalizes the timestamp to a canonical format.
    ///
    /// Based on [`google::protobuf::util::CreateNormalized`][1].
    ///
    /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L59-L77
    pub fn normalize(&mut self) {
        if self.nanos <= -NANOS_PER_SECOND || self.nanos >= NANOS_PER_SECOND {
            if let Some(seconds) = self
                .seconds
                .checked_add((self.nanos / NANOS_PER_SECOND) as i64)
            {
                self.seconds = seconds;
                self.nanos %= NANOS_PER_SECOND;
            } else if self.nanos < 0 {
                self.seconds = i64::MIN;
                self.nanos = 0;
            } else {
                self.seconds = i64::MAX;
                self.nanos = 999_999_999;
            }
        }
        if self.nanos < 0 {
            if let Some(seconds) = self.seconds.checked_sub(1) {
                self.seconds = seconds;
                self.nanos += NANOS_PER_SECOND;
            } else {
                if true {
                    match (&self.seconds, &i64::MIN) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                };
                self.nanos = 0;
            }
        }
    }
    /// Creates a new `Timestamp` at the start of the provided UTC date.
    pub fn date(year: i64, month: u8, day: u8) -> Result<Timestamp, TimestampError> {
        Timestamp::date_time_nanos(year, month, day, 0, 0, 0, 0)
    }
    /// Creates a new `Timestamp` instance with the provided UTC date and time.
    pub fn date_time(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Timestamp, TimestampError> {
        Timestamp::date_time_nanos(year, month, day, hour, minute, second, 0)
    }
    /// Creates a new `Timestamp` instance with the provided UTC date and time.
    pub fn date_time_nanos(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanos: u32,
    ) -> Result<Timestamp, TimestampError> {
        let date_time = datetime::DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanos,
        };
        if date_time.is_valid() {
            Ok(Timestamp::from(date_time))
        } else {
            Err(TimestampError::InvalidDateTime)
        }
    }
}
/// Implements the unstable/naive version of `Eq`: a basic equality check on the internal fields of the `Timestamp`.
/// This implies that `normalized_ts != non_normalized_ts` even if `normalized_ts == non_normalized_ts.normalized()`.
#[cfg(feature = "std")]
impl Eq for Timestamp {}
#[cfg(feature = "std")]
#[allow(clippy::derive_hash_xor_eq)]
impl std::hash::Hash for Timestamp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.seconds.hash(state);
        self.nanos.hash(state);
    }
}
#[cfg(feature = "std")]
impl From<std::time::SystemTime> for Timestamp {
    fn from(system_time: std::time::SystemTime) -> Timestamp {
        let (seconds, nanos) = match system_time.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                let seconds = i64::try_from(duration.as_secs()).unwrap();
                (seconds, duration.subsec_nanos() as i32)
            }
            Err(error) => {
                let duration = error.duration();
                let seconds = i64::try_from(duration.as_secs()).unwrap();
                let nanos = duration.subsec_nanos() as i32;
                if nanos == 0 {
                    (-seconds, 0)
                } else {
                    (-seconds - 1, 1_000_000_000 - nanos)
                }
            }
        };
        Timestamp { seconds, nanos }
    }
}
/// A timestamp handling error.
#[non_exhaustive]
pub enum TimestampError {
    /// Indicates that a [`Timestamp`] could not be converted to
    /// [`SystemTime`][std::time::SystemTime] because it is out of range.
    ///
    /// The range of times that can be represented by `SystemTime` depends on the platform. All
    /// `Timestamp`s are likely representable on 64-bit Unix-like platforms, but other platforms,
    /// such as Windows and 32-bit Linux, may not be able to represent the full range of
    /// `Timestamp`s.
    OutOfSystemRange(Timestamp),
    /// An error indicating failure to parse a timestamp in RFC-3339 format.
    ParseFailure,
    /// Indicates an error when constructing a timestamp due to invalid date or time data.
    InvalidDateTime,
}
#[automatically_derived]
impl ::core::fmt::Debug for TimestampError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TimestampError::OutOfSystemRange(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "OutOfSystemRange", &__self_0)
            }
            TimestampError::ParseFailure => ::core::fmt::Formatter::write_str(f, "ParseFailure"),
            TimestampError::InvalidDateTime => {
                ::core::fmt::Formatter::write_str(f, "InvalidDateTime")
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TimestampError {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TimestampError {
    #[inline]
    fn eq(&self, other: &TimestampError) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    TimestampError::OutOfSystemRange(__self_0),
                    TimestampError::OutOfSystemRange(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => true,
            }
    }
}
impl fmt::Display for TimestampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimestampError::OutOfSystemRange(timestamp) => {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &[
                        "",
                        " is not representable as a `SystemTime` because it is out of range",
                    ],
                    &[::core::fmt::ArgumentV1::new_display(&timestamp)],
                ))
            }
            TimestampError::ParseFailure => f.write_fmt(::core::fmt::Arguments::new_v1(
                &["failed to parse RFC-3339 formatted timestamp"],
                &[],
            )),
            TimestampError::InvalidDateTime => f.write_fmt(::core::fmt::Arguments::new_v1(
                &["invalid date or time"],
                &[],
            )),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for TimestampError {}
#[cfg(feature = "std")]
impl TryFrom<Timestamp> for std::time::SystemTime {
    type Error = TimestampError;
    fn try_from(mut timestamp: Timestamp) -> Result<std::time::SystemTime, Self::Error> {
        let orig_timestamp = timestamp.clone();
        timestamp.normalize();
        let system_time = if timestamp.seconds >= 0 {
            std::time::UNIX_EPOCH.checked_add(time::Duration::from_secs(timestamp.seconds as u64))
        } else {
            std::time::UNIX_EPOCH.checked_sub(time::Duration::from_secs(
                timestamp
                    .seconds
                    .checked_neg()
                    .ok_or_else(|| TimestampError::OutOfSystemRange(timestamp.clone()))?
                    as u64,
            ))
        };
        let system_time = system_time.and_then(|system_time| {
            system_time.checked_add(time::Duration::from_nanos(timestamp.nanos as u64))
        });
        system_time.ok_or(TimestampError::OutOfSystemRange(orig_timestamp))
    }
}
impl FromStr for Timestamp {
    type Err = TimestampError;
    fn from_str(s: &str) -> Result<Timestamp, TimestampError> {
        datetime::parse_timestamp(s).ok_or(TimestampError::ParseFailure)
    }
}
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        datetime::DateTime::from(self.clone()).fmt(f)
    }
}
