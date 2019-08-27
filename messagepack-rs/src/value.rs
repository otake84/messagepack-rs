use crate::binary::Binary;
use crate::deserializable::Deserializable;
use crate::extension::Extension;
use crate::serializable::{Serializable, SerializeError};
use chrono::prelude::*;
use messagepack_rs_macros::MessagePackFrom;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, MessagePackFrom)]
pub enum Value {
    Nil,
    Bool(bool),
    Float32(f32),
    Float64(f64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Binary(Binary),
    String(String),
    Array(Vec<Self>),
    Map(BTreeMap<String, Self>),
    Extension(Extension),
    Timestamp(DateTime<Utc>),
}

impl Serializable for Value {
    fn serialize(self) -> Result<Vec<u8>, SerializeError> {
        match self {
            Self::Nil => Self::serialize_nil(),
            Self::Bool(v) => Self::serialize_bool(v),
            Self::Float32(v) => Self::serialize_float32(v),
            Self::Float64(v) => Self::serialize_float64(v),
            Self::UInt8(v) => Self::serialize_uint8(v),
            Self::UInt16(v) => Self::serialize_uint16(v),
            Self::UInt32(v) => Self::serialize_uint32(v),
            Self::UInt64(v) => Self::serialize_uint64(v),
            Self::Int8(v) => Self::serialize_int8(v),
            Self::Int16(v) => Self::serialize_int16(v),
            Self::Int32(v) => Self::serialize_int32(v),
            Self::Int64(v) => Self::serialize_int64(v),
            Self::Binary(v) => Self::serialize_binary(v),
            Self::String(v) => Self::serialize_string(v),
            Self::Array(v) => Self::serialize_array(v),
            Self::Map(v) => Self::serialize_map(v),
            Self::Extension(v) => Self::serialize_extension(v),
            Self::Timestamp(v) => Self::serialize_timestamp(v),
        }
    }
}

impl Deserializable for Value {}
