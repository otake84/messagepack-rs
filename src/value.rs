use crate::binary::Binary;
use crate::deserializable::Deserializable;
use crate::extension::Extension;
use crate::serializable::{Serializable, SerializeError};
use chrono::prelude::*;
use std::collections::BTreeMap;
use std::convert::{From, Into};

#[derive(Clone, Debug, PartialEq)]
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

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        value.map_or(Self::Nil, Into::into)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::UInt8(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::UInt16(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::UInt32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::UInt64(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::Int8(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Int16(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int64(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Binary> for Value {
    fn from(value: Binary) -> Self {
        Self::Binary(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(String::from(value))
    }
}

impl From<Vec<Self>> for Value {
    fn from(value: Vec<Self>) -> Self {
        Self::Array(value)
    }
}

impl From<&[Self]> for Value {
    fn from(value: &[Self]) -> Self {
        Self::Array(Vec::from(value))
    }
}

impl From<BTreeMap<String, Self>> for Value {
    fn from(value: BTreeMap<String, Self>) -> Self {
        Self::Map(value)
    }
}

impl From<Extension> for Value {
    fn from(value: Extension) -> Self {
        Self::Extension(value)
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(value: DateTime<Utc>) -> Self {
        Self::Timestamp(value)
    }
}
