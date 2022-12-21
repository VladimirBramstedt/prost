//! Utility functions and types for encoding and decoding Protobuf types.
//!
//! Meant to be used only from `Message` implementations.

#![allow(clippy::implicit_hasher, clippy::ptr_arg)]

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use core::marker::PhantomData;
use core::u32;
use core::usize;
use std::collections::HashMap;

use ::bytes::{Buf, BufMut, Bytes};

use crate::encoding::WireType;
use crate::encoding::*;
use crate::DecodeError;
use crate::Message;
/*
Protobuf schema types
*/

trait ProtobufType {}
impl ProtobufType for Int32 {}
pub struct Int32;
pub struct Uint32;
pub struct Sint32;
impl ProtobufType for Sint32 {}
pub struct Fixed32;
impl ProtobufType for Fixed32 {}
pub struct SFixed32;
impl ProtobufType for SFixed32 {}
pub struct Int64;
pub struct Uint64;

pub struct Sint64;
pub struct Fixed64;
pub struct SFixed64;

pub struct ProtobufFloat;
pub struct ProtobufDouble;

pub struct ProtobufBool;
pub struct ProtobufString;
pub struct ProtobufBytes;

pub struct ProtobufEnum;
pub struct ProtobufOneOf;
pub struct ProtobufMessage;
pub struct ProtobufMap;

pub struct Enumeration<E> {
    pub(crate) raw: i32,
    repr: PhantomData<E>,
}

impl<E: TryFrom<i32> + Default> Enumeration<E> {
    pub fn as_enum(&self) -> Option<E> {
        self.raw.try_into().ok()
    }
    pub fn as_i32(&self) -> i32 {
        self.raw
    }
    pub fn from_i32(raw: i32) -> Self {
        Self {
            raw,
            repr: PhantomData,
        }
    }
}
// Derive Enumeration should create the enum, derive Default for the first variant, and derive From/Into i32.
enum SomeEnum {
    One = 1,
    Two = 2,
    Three = 42,
}
impl Default for SomeEnum {
    fn default() -> Self {
        SomeEnum::One
    }
}
impl<E: Default + Into<i32>> Default for Enumeration<E> {
    fn default() -> Self {
        let raw = E::default().into();

        Self {
            raw,
            repr: PhantomData,
        }
    }
}
impl<E: Default + TryFrom<i32>> AsProtoValue<ProtobufEnum> for Enumeration<E> {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(self.as_i32() as u64, buffer)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        _ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let r = &mut value.raw;
        *r = decode_varint(buffer)? as i32;
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        encoded_len_varint(self.as_i32() as u64)
    }
}

/// Anything you can Map to the protobuf type system Above
pub trait AsProtoValue<T> {
    const WIRE_TYPE: WireType;
    // just write the (length delimited) value to the buffer. no tag.
    fn raw_encode(&self, buffer: &mut impl BufMut);
    // len of value, sans key
    fn encode_key(tag: u32, buffer: &mut impl BufMut);
    fn encoded_len(&self) -> usize;
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>;
}

impl<M> AsProtoValue<ProtobufMessage> for M
where
    M: Message,
{
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(self.encoded_len() as u64, buffer);
        self.encode_raw(buffer);
        // // message::encode(tag, self, buffer)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::LengthDelimited, buffer)
    }
    fn encoded_len(&self) -> usize {
        self.encoded_len()
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        message::merge(wire_type, value, buffer, ctx)
    }
}

impl AsProtoValue<ProtobufEnum> for i32 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(*self as u64, buffer)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        int32::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        encoded_len_varint(*self as u64)
    }
}

impl AsProtoValue<Int32> for i32 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(*self as u64, buffer)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,

        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        int32::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        encoded_len_varint(*self as u64)
    }
}
impl AsProtoValue<ProtobufBool> for bool {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(*self as u64, buffer)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn merge(
        _wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        _ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let b = decode_varint(buffer)?;

        *value = b != 0;
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        encoded_len_varint(*self as u64)
    }
}
impl AsProtoValue<Sint32> for i32 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(((self << 1) ^ (self >> 31)) as u32 as u64, buffer);
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn encoded_len(&self) -> usize {
        encoded_len_varint(((self << 1) ^ (self >> 31)) as u32 as u64)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        sint32::merge(wire_type, value, buffer, ctx)
    }
}

impl AsProtoValue<SFixed32> for i32 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        buffer.put_i32_le(*self)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::ThirtyTwoBit, buffer)
    }
    fn encoded_len(&self) -> usize {
        4
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        sfixed32::merge(wire_type, value, buffer, ctx)
    }
}

impl AsProtoValue<Fixed32> for u32 {
    const WIRE_TYPE: WireType = WireType::ThirtyTwoBit;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        buffer.put_u32_le(*self)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::ThirtyTwoBit, buffer)
    }
    fn encoded_len(&self) -> usize {
        4
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        fixed32::merge(wire_type, value, buffer, ctx)
    }
}
impl AsProtoValue<Uint32> for u32 {
    const WIRE_TYPE: WireType = WireType::ThirtyTwoBit;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(*self as u64, buffer)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn encoded_len(&self) -> usize {
        4
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        uint32::merge(wire_type, value, buffer, ctx)
    }
}

impl AsProtoValue<Int64> for i64 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(*self as u64, buffer)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        int64::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        encoded_len_varint(*self as u64)
    }
}

impl AsProtoValue<Sint64> for i64 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(((self << 1) ^ (self >> 63)) as u64, buffer);
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        sint64::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        encoded_len_varint(((self << 1) ^ (self >> 63)) as u64)
    }
}

impl AsProtoValue<SFixed64> for i64 {
    const WIRE_TYPE: WireType = WireType::SixtyFourBit;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        buffer.put_i64_le(*self)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::SixtyFourBit, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        sfixed64::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        8
    }
}

impl AsProtoValue<Fixed64> for u64 {
    const WIRE_TYPE: WireType = WireType::SixtyFourBit;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        buffer.put_u64_le(*self)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::SixtyFourBit, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        fixed64::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        8
    }
}

impl AsProtoValue<Uint64> for u64 {
    const WIRE_TYPE: WireType = WireType::SixtyFourBit;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(*self, buffer)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::Varint, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        uint64::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        8
    }
}
impl AsProtoValue<ProtobufDouble> for f64 {
    const WIRE_TYPE: WireType = WireType::SixtyFourBit;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        buffer.put_f64_le(*self)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::SixtyFourBit, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        double::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        8
    }
}

impl AsProtoValue<ProtobufFloat> for f32 {
    const WIRE_TYPE: WireType = WireType::ThirtyTwoBit;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        buffer.put_f32_le(*self)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::ThirtyTwoBit, buffer)
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        float::merge(wire_type, value, buffer, ctx)
    }

    fn encoded_len(&self) -> usize {
        4
    }
}

impl AsProtoValue<ProtobufString> for String {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(self.len() as u64, buffer);
        buffer.put_slice(self.as_bytes())
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::LengthDelimited, buffer)
    }
    fn encoded_len(&self) -> usize {
        self.len()
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        string::merge(wire_type, value, buffer, ctx)
    }
}

impl AsProtoValue<ProtobufBytes> for Vec<u8> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(self.len() as u64, buffer);
        buffer.put_slice(self)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::LengthDelimited, buffer)
    }
    fn encoded_len(&self) -> usize {
        self.len()
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        bytes::merge(wire_type, value, buffer, ctx)
    }
}

impl AsProtoValue<ProtobufBytes> for Bytes {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
    fn raw_encode(&self, buffer: &mut impl BufMut) {
        encode_varint(self.len() as u64, buffer);
        buffer.put_slice(self)
    }
    fn encode_key(tag: u32, buffer: &mut impl BufMut) {
        encode_key(tag, WireType::LengthDelimited, buffer)
    }
    fn encoded_len(&self) -> usize {
        self.len()
    }
    fn merge(
        wire_type: WireType,
        value: &mut Self,
        buffer: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        bytes::merge(wire_type, value, buffer, ctx)
    }
}

pub struct Repeated;
pub struct Optional;
pub struct Required;
pub struct Plain;
pub struct Packed;

pub trait Mode {
    type Container<T>;
    fn encode<P, T: AsProtoValue<P>, B: BufMut>(
        tag: u32,
        values: &Self::Container<T>,
        buffer: &mut B,
    );
    // fn encoded_len<P, T: AsProtoValue<P>, B: BufMut>(
    //     tag: u32,
    //     values: &Self::Container<T>,
    // ) -> usize;

    fn merge<P, T: AsProtoValue<P> + Default, B: Buf>(
        wire_type: WireType,
        values: &mut Self::Container<T>,
        buffer: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>;
}

pub trait Mode2 {
    type Container<T>;
    fn encode<P, T: AsProtoValue<P>, B: BufMut>(
        tag: u32,
        values: &Self::Container<T>,
        buffer: &mut B,
    );
    // fn encoded_len<P, T: AsProtoValue<P>, B: BufMut>(
    //     tag: u32,
    //     values: &Self::Container<T>,
    // ) -> usize;

    fn merge<P, T: AsProtoValue<P> + Default, B: Buf>(
        wire_type: WireType,
        values: &mut Self::Container<T>,
        buffer: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>;
}

impl Mode for Repeated {
    type Container<T> = Vec<T>;

    fn encode<P, T: AsProtoValue<P>, B: BufMut>(
        tag: u32,

        values: &Self::Container<T>,
        buffer: &mut B,
    ) {
        for v in values {
            T::encode_key(tag, buffer);
            AsProtoValue::<P>::raw_encode(v, buffer);
        }
    }

    fn merge<P, T: AsProtoValue<P> + Default, B: Buf>(
        wire_type: WireType,
        values: &mut Self::Container<T>,
        buffer: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut val = T::default();
        AsProtoValue::<P>::merge(wire_type, &mut val, buffer, ctx)?;
        Ok(values.push(val))
    }
}

impl Mode for Packed {
    type Container<T> = Vec<T>;

    fn encode<P, T: AsProtoValue<P>, B: BufMut>(
        tag: u32,

        values: &Self::Container<T>,
        buffer: &mut B,
    ) {
        if values.is_empty() {
            return;
        }

        encode_key(tag, WireType::LengthDelimited, buffer);
        let len: usize = values.iter().map(|v| v.encoded_len()).sum();
        encode_varint(len as u64, buffer);

        for v in values {
            v.raw_encode(buffer)
        }
    }
    fn merge<P, T: AsProtoValue<P> + Default, B: Buf>(
        wire_type: WireType,
        values: &mut Self::Container<T>,
        buffer: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        // assert_eq!(wire_type, WireType::LengthDelimited);
        let len = decode_varint(buffer)?;
        let remaining = buffer.remaining();
        if len > remaining as u64 {
            return Err(DecodeError::new("buffer underflow"));
        }

        let limit = remaining - len as usize;
        let inner_wire = T::WIRE_TYPE;
        while buffer.remaining() > limit {
            let mut val = T::default();
            AsProtoValue::<P>::merge(inner_wire, &mut val, buffer, ctx.clone())?;
            values.push(val)
            // merge(value, buf, ctx.clone())?;
        }

        if buffer.remaining() != limit {
            return Err(DecodeError::new("delimited length exceeded"));
        }
        Ok(())
        // merge_loop(values, buffer, ctx, |value, buffer, context| {
        //     let mut val = T::default();
        //     AsProtoValue::<P>::merge(wire_type, value, buffer, context)?;
        //     Ok(values.push(val))
        // })
    }
}

pub fn merge_loop<T, M, B>(
    value: &mut T,
    buf: &mut B,
    ctx: DecodeContext,
    mut merge: M,
) -> Result<(), DecodeError>
where
    M: FnMut(&mut T, &mut B, DecodeContext) -> Result<(), DecodeError>,
    B: Buf,
{
    let len = decode_varint(buf)?;
    let remaining = buf.remaining();
    if len > remaining as u64 {
        return Err(DecodeError::new("buffer underflow"));
    }

    let limit = remaining - len as usize;
    while buf.remaining() > limit {
        merge(value, buf, ctx.clone())?;
    }

    if buf.remaining() != limit {
        return Err(DecodeError::new("delimited length exceeded"));
    }
    Ok(())
}
impl Mode for Optional {
    type Container<T> = Option<T>;

    fn encode<P, T: AsProtoValue<P>, B: BufMut>(
        tag: u32,
        values: &Self::Container<T>,
        buffer: &mut B,
    ) {
        match values {
            Some(v) => {
                T::encode_key(tag, buffer);
                AsProtoValue::<P>::raw_encode(v, buffer)
            }
            None => (),
        }
    }
    fn merge<P, T: AsProtoValue<P> + Default, B: Buf>(
        wire_type: WireType,
        values: &mut Self::Container<T>,
        buffer: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        match values {
            Some(val) => AsProtoValue::<P>::merge(wire_type, val, buffer, ctx)?,
            None => {
                let mut val = T::default();
                AsProtoValue::<P>::merge(wire_type, &mut val, buffer, ctx)?;
                *values = Some(val);
            }
        }

        Ok(())
    }
}

impl Mode for Required {
    type Container<T> = T;

    fn encode<P, T: AsProtoValue<P>, B: BufMut>(
        tag: u32,
        values: &Self::Container<T>,
        buffer: &mut B,
    ) {
        T::encode_key(tag, buffer);
        AsProtoValue::<P>::raw_encode(values, buffer)
    }
    fn merge<P, T: AsProtoValue<P> + Default, B: Buf>(
        wire_type: WireType,
        values: &mut Self::Container<T>,
        buffer: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut val = T::default();
        AsProtoValue::<P>::merge(wire_type, &mut val, buffer, ctx)?;
        *values = val;
        Ok(())
    }
}

impl Mode for Plain {
    type Container<T> = T;

    fn encode<P, T: AsProtoValue<P>, B: BufMut>(
        tag: u32,
        values: &Self::Container<T>,
        buffer: &mut B,
    ) {
        T::encode_key(tag, buffer);
        AsProtoValue::<P>::raw_encode(values, buffer)
    }
    fn merge<P, T: AsProtoValue<P> + Default, B: Buf>(
        wire_type: WireType,
        values: &mut Self::Container<T>,
        buffer: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut val = T::default();
        AsProtoValue::<P>::merge(wire_type, &mut val, buffer, ctx)?;
        *values = val;
        Ok(())
    }
}

trait ProtoMap<K, V> {
    fn encode_map(&self, tag: u32, buffer: &mut impl BufMut);
    fn merge_map();
}

impl<A, B, K, V> ProtoMap<K, V> for BTreeMap<A, B>
where
    A: Default + AsProtoValue<K> + PartialEq,
    B: Default + AsProtoValue<V> + PartialEq,
{
    fn encode_map(&self, tag: u32, buffer: &mut impl BufMut) {
        for (k, v) in self.iter() {
            let skip_key = k == &A::default();
            let skip_val = v == &B::default();
            let key_len = if skip_key {
                0
            } else {
                AsProtoValue::<K>::encoded_len(k)
            };
            let val_len = if skip_val {
                0
            } else {
                AsProtoValue::<V>::encoded_len(v)
            };
            encode_key(tag, WireType::LengthDelimited, buffer);
            encode_varint((key_len + val_len) as u64, buffer);
            if !skip_key {
                AsProtoValue::<K>::raw_encode(k, buffer);
                // key_encode(1, key, buf);
            }
            if !skip_val {
                AsProtoValue::<V>::raw_encode(v, buffer);
                // val_encode(2, val, buf);
            }
        }
    }

    fn merge_map() {
        todo!()
    }
}

// pub fn encode_with_default<K, V, B, KE, KL, VE, VL>(
//     key_encode: KE,
//     key_encoded_len: KL,
//     val_encode: VE,
//     val_encoded_len: VL,
//     val_default: &V,
//     tag: u32,
//     values: &$map_ty<K, V>,
//     buf: &mut B,
// ) where
//     K: Default + Eq + Hash + Ord,
//     V: PartialEq,
//     B: BufMut,
//     KE: Fn(u32, &K, &mut B),
//     KL: Fn(u32, &K) -> usize,
//     VE: Fn(u32, &V, &mut B),
//     VL: Fn(u32, &V) -> usize,
// {
//     for (key, val) in values.iter() {
//         let skip_key = key == &K::default();
//         let skip_val = val == val_default;

//         let len = (if skip_key { 0 } else { key_encoded_len(1, key) })
//             + (if skip_val { 0 } else { val_encoded_len(2, val) });

//         encode_key(tag, WireType::LengthDelimited, buf);
//         encode_varint(len as u64, buf);
//         if !skip_key {
//             key_encode(1, key, buf);
//         }
//         if !skip_val {
//             val_encode(2, val, buf);
//         }
//     }
// }
// pub fn merge_with_default<K, V, B, KM, VM>(
//     key_merge: KM,
//     val_merge: VM,
//     values: &mut $map_ty<K, V>,
//     buf: &mut B,
//     ctx: DecodeContext,
// ) -> Result<(), DecodeError>
// where
//     K: Default + Eq + Hash + Ord,
//     B: Buf,
//     KM: Fn(WireType, &mut K, &mut B, DecodeContext) -> Result<(), DecodeError>,
//     VM: Fn(WireType, &mut V, &mut B, DecodeContext) -> Result<(), DecodeError>,
// {
//     let mut key = Default::default();
//     let mut val = Default::default();
//     ctx.limit_reached()?;
//     merge_loop(
//         &mut (&mut key, &mut val),
//         buf,
//         ctx.enter_recursion(),
//         |&mut (ref mut key, ref mut val), buf, ctx| {
//             let (tag, wire_type) = decode_key(buf)?;
//             match tag {
//                 1 => key_merge(wire_type, key, buf, ctx),
//                 2 => val_merge(wire_type, val, buf, ctx),
//                 _ => skip_field(wire_type, tag, buf, ctx),
//             }
//         },
//     )?;
//     values.insert(key, val);

//     Ok(())
// }
