use byteorder::{ReadBytesExt, WriteBytesExt};
use chrono::prelude::*;
use messagepack_rs::binary::Binary;
use messagepack_rs::deserializable::{Deserializable, DeserializeError};
use messagepack_rs::extension::Extension;
use messagepack_rs::marker::Marker;
use messagepack_rs::serializable::{Serializable, SerializeError};
use messagepack_rs_macros::MessagePackFrom;
use std::collections::BTreeMap;
use std::io::{BufReader, Cursor, Read, Seek};

#[derive(Clone, Debug, PartialEq)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Clone, Debug, PartialEq, MessagePackFrom)]
enum MyValue {
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
    Rgba(Rgba),
}

impl From<Rgba> for MyValue {
    fn from(value: Rgba) -> Self {
        Self::Rgba(value)
    }
}

impl Serializable for MyValue {
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
            Self::Rgba(v) => {
                let mut w = Vec::with_capacity(1 + 1 + 4);
                w.write_u8(Marker::FixExt4.into()).or(Err(SerializeError::FailedToWrite))?;
                w.write_i8(0).or(Err(SerializeError::FailedToWrite))?;
                w.write_u8(v.r).or(Err(SerializeError::FailedToWrite))?;
                w.write_u8(v.g).or(Err(SerializeError::FailedToWrite))?;
                w.write_u8(v.b).or(Err(SerializeError::FailedToWrite))?;
                w.write_u8(v.a).or(Err(SerializeError::FailedToWrite))?;
                Ok(w)
            },
        }
    }
}

impl Deserializable for MyValue {
    fn deserialize_extension_for_the_you_type_defined<R: Read + Seek>(t: i8, size: usize, buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        if t == 0 {
            Ok(From::from(Rgba {
                r: buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?,
                g: buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?,
                b: buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?,
                a: buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?,
            }))
        } else {
            Self::deserialize_extension_others(t, size, buf_reader)
        }
    }
}

fn main() {
    let rgba = Rgba { r: 5, g: 10, b: 15, a: 20 };
    let value = MyValue::from(rgba);
    println!("{:?}", value);

    let serialized_value = value.serialize().unwrap();
    println!("{:?}", serialized_value);

    let deserialized_value = MyValue::deserialize(&mut BufReader::new(Cursor::new(serialized_value))).unwrap();
    println!("{:?}", deserialized_value);
}
