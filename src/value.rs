use crate::binary::Binary;
use crate::marker::Marker;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::prelude::*;
use std::collections::BTreeMap;
use std::convert::{From, Into};
use std::io::{BufReader, Read, Write};

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
    Extension(i8, Vec<u8>),
    Timestamp(DateTime<Utc>),
}

#[derive(Debug)]
pub enum SerializeError {
    FailedToWrite,
    OutOfRange,
}

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength,
    InvalidMarker,
    InvalidValue,
}

impl Value {
    pub fn serialize(self) -> Result<Vec<u8>, SerializeError> {
        match self {
            Value::Nil => Ok(vec![Marker::Nil.into()]),
            Value::Bool(v) => Ok(if v { vec![Marker::True.into()] } else { vec![Marker::False.into()] }),
            Value::Float32(v) => {
                let mut w = Vec::with_capacity(1 + 4);
                w.write_u8(Marker::Float32.into()).or(Err(SerializeError::FailedToWrite))?;
                w.write_f32::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
                Ok(w)
            },
            Value::Float64(v) => {
                let mut w = Vec::with_capacity(1 + 8);
                w.write_u8(Marker::Float64.into()).or(Err(SerializeError::FailedToWrite))?;
                w.write_f64::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
                Ok(w)
            },
            Value::UInt8(v) => {
                if v < 0b1000_0000 {
                    let mut w = Vec::with_capacity(1);
                    w.write_u8(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else {
                    let mut w = Vec::with_capacity(1 + 1);
                    w.write_u8(Marker::UInt8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u8(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
            Value::UInt16(v) => {
                if v < 0b1000_0000 {
                    let mut w = Vec::with_capacity(1);
                    w.write_u8(v as u8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v <= u16::from(u8::max_value()) {
                    let mut w = Vec::with_capacity(1 + 1);
                    w.write_u8(Marker::UInt8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u8(v as u8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else {
                    let mut w = Vec::with_capacity(1 + 2);
                    w.write_u8(Marker::UInt16.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u16::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
            Value::UInt32(v) => {
                if v < 0b1000_0000 {
                    let mut w = Vec::with_capacity(1);
                    w.write_u8(v as u8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v <= u32::from(u8::max_value()) {
                    let mut w = Vec::with_capacity(1 + 1);
                    w.write_u8(Marker::UInt8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u8(v as u8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v <= u32::from(u16::max_value()) {
                    let mut w = Vec::with_capacity(1 + 2);
                    w.write_u8(Marker::UInt16.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u16::<BigEndian>(v as u16).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else {
                    let mut w = Vec::with_capacity(1 + 4);
                    w.write_u8(Marker::UInt32.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u32::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
            Value::UInt64(v) => {
                if v < 0b1000_0000 {
                    let mut w = Vec::with_capacity(1);
                    w.write_u8(v as u8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v <= u64::from(u8::max_value()) {
                    let mut w = Vec::with_capacity(1 + 1);
                    w.write_u8(Marker::UInt8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u8(v as u8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v <= u64::from(u16::max_value()) {
                    let mut w = Vec::with_capacity(1 + 2);
                    w.write_u8(Marker::UInt16.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u16::<BigEndian>(v as u16).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v <= u64::from(u32::max_value()) {
                    let mut w = Vec::with_capacity(1 + 4);
                    w.write_u8(Marker::UInt32.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u32::<BigEndian>(v as u32).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else {
                    let mut w = Vec::with_capacity(1 + 8);
                    w.write_u8(Marker::UInt64.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u64::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
            Value::Int8(v) => {
                if v >= -32 && v <= -1 {
                    let mut w = Vec::with_capacity(1);
                    w.write_i8(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else {
                    let mut w = Vec::with_capacity(1 + 1);
                    w.write_u8(Marker::Int8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i8(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
            Value::Int16(v) => {
                if v >= -32 && v <= -1 {
                    let mut w = Vec::with_capacity(1);
                    w.write_i8(v as i8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v >= i16::from(i8::min_value()) && v <= i16::from(i8::max_value()) {
                    let mut w = Vec::with_capacity(1 + 1);
                    w.write_u8(Marker::Int8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i8(v as i8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else {
                    let mut w = Vec::with_capacity(1 + 2);
                    w.write_u8(Marker::Int16.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i16::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
            Value::Int32(v) => {
                if v >= -32 && v <= -1 {
                    let mut w = Vec::with_capacity(1);
                    w.write_i8(v as i8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v >= i32::from(i8::min_value()) && v <= i32::from(i8::max_value()) {
                    let mut w = Vec::with_capacity(1 + 1);
                    w.write_u8(Marker::Int8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i8(v as i8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v >= i32::from(i16::min_value()) && v <= i32::from(i16::max_value()) {
                    let mut w = Vec::with_capacity(1 + 2);
                    w.write_u8(Marker::Int16.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i16::<BigEndian>(v as i16).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else {
                    let mut w = Vec::with_capacity(1 + 4);
                    w.write_u8(Marker::Int32.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i32::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
            Value::Int64(v) => {
                if v >= -32 && v <= -1 {
                    let mut w = Vec::with_capacity(1);
                    w.write_i8(v as i8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v >= i64::from(i8::min_value()) && v <= i64::from(i8::max_value()) {
                    let mut w = Vec::with_capacity(1 + 1);
                    w.write_u8(Marker::Int8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i8(v as i8).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v >= i64::from(i16::min_value()) && v <= i64::from(i16::max_value()) {
                    let mut w = Vec::with_capacity(1 + 2);
                    w.write_u8(Marker::Int16.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i16::<BigEndian>(v as i16).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else if v >= i64::from(i32::min_value()) && v <= i64::from(i32::max_value()) {
                    let mut w = Vec::with_capacity(1 + 4);
                    w.write_u8(Marker::Int32.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i32::<BigEndian>(v as i32).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                } else {
                    let mut w = Vec::with_capacity(1 + 8);
                    w.write_u8(Marker::Int64.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i64::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
            Value::Binary(v) => {
                let mut w = match v.0.len() {
                    len if u8::max_value() as usize >= len => {
                        let mut w = Vec::with_capacity(1 + 1 + len);
                        w.write_u8(Marker::Bin8.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u8(len as u8).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if u16::max_value() as usize >= len => {
                        let mut w = Vec::with_capacity(1 + 2 + len);
                        w.write_u8(Marker::Bin16.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u16::<BigEndian>(len as u16).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if u32::max_value() as usize >= len => {
                        let mut w = Vec::with_capacity(1 + 4 + len);
                        w.write_u8(Marker::Bin32.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u32::<BigEndian>(len as u32).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    _ => Err(SerializeError::OutOfRange)?,
                };
                w.write_all(&v.0).or(Err(SerializeError::FailedToWrite))?;
                Ok(w)
            },
            Value::String(v) => {
                let mut w = match v.len() {
                    len if len <= 31 => {
                        let mut w = Vec::with_capacity(1 + len);
                        w.write_u8(Marker::FixStr(len as u8).into()).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if u8::max_value() as usize >= len => {
                        let mut w = Vec::with_capacity(1 + 1 + len);
                        w.write_u8(Marker::Str8.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u8(len as u8).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if u16::max_value() as usize >= len => {
                        let mut w = Vec::with_capacity(1 + 2 + len);
                        w.write_u8(Marker::Str16.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u16::<BigEndian>(len as u16).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if u32::max_value() as usize >= len => {
                        let mut w = Vec::with_capacity(1 + 4 + len);
                        w.write_u8(Marker::Str32.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u32::<BigEndian>(len as u32).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    _ => Err(SerializeError::OutOfRange)?,
                };
                w.write_all(v.as_bytes()).or(Err(SerializeError::FailedToWrite))?;
                Ok(w)
            },
            Value::Array(v) => {
                let mut values = Vec::new();
                let len = v.len();
                for vv in v.into_iter() {
                    values.append(&mut Value::serialize(vv)?);
                }
                let mut w = match len {
                    len if len <= 15 => {
                        let mut w = Vec::with_capacity(1 + values.len());
                        w.write_u8(Marker::FixArray(len as u8).into()).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if u16::max_value() as usize >= len => {
                        let mut w = Vec::with_capacity(1 + 2 + values.len());
                        w.write_u8(Marker::Array16.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u16::<BigEndian>(len as u16).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if u32::max_value() as usize >= len => {
                        let mut w = Vec::with_capacity(1 + 4 + values.len());
                        w.write_u8(Marker::Array32.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u32::<BigEndian>(len as u32).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    _ => Err(SerializeError::OutOfRange)?,
                };
                w.append(&mut values);
                Ok(w)
            },
            Value::Map(v) => {
                let mut w = match v.len() {
                    len if len <= 15 => {
                        vec![Marker::FixMap(len as u8).into()]
                    },
                    len if u16::max_value() as usize >= len => {
                        let mut w = vec![Marker::Map16.into()];
                        w.write_u16::<BigEndian>(len as u16).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if u32::max_value() as usize >= len => {
                        let mut w = vec![Marker::Map32.into()];
                        w.write_u32::<BigEndian>(len as u32).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    _ => Err(SerializeError::OutOfRange)?,
                };
                for (k, v) in v {
                    w.write_all(&Value::String(k).serialize()?).or(Err(SerializeError::FailedToWrite))?;
                    w.write_all(&v.serialize()?).or(Err(SerializeError::FailedToWrite))?;
                }
                Ok(w)
            },
            Value::Extension(t, mut v) => {
                let mut w = match v.len() {
                    1 => {
                        let mut w = Vec::with_capacity(1 + 1 + 1);
                        w.write_u8(Marker::FixExt1.into()).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    2 => {
                        let mut w = Vec::with_capacity(1 + 1 + 2);
                        w.write_u8(Marker::FixExt2.into()).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    4 => {
                        let mut w = Vec::with_capacity(1 + 1 + 4);
                        w.write_u8(Marker::FixExt4.into()).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    8 => {
                        let mut w = Vec::with_capacity(1 + 1 + 8);
                        w.write_u8(Marker::FixExt8.into()).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    16 => {
                        let mut w = Vec::with_capacity(1 + 1 + 16);
                        w.write_u8(Marker::FixExt16.into()).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if len <= u8::max_value() as usize => {
                        let mut w = Vec::with_capacity(1 + 1 + 1 + len);
                        w.write_u8(Marker::Ext8.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u8(len as u8).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if len <= u16::max_value() as usize => {
                        let mut w = Vec::with_capacity(1 + 2 + 1 + len);
                        w.write_u8(Marker::Ext16.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u16::<BigEndian>(len as u16).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    len if len <= u32::max_value() as usize => {
                        let mut w = Vec::with_capacity(1 + 4 + 1 + len);
                        w.write_u8(Marker::Ext32.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u32::<BigEndian>(len as u32).or(Err(SerializeError::FailedToWrite))?;
                        w
                    },
                    _ => Err(SerializeError::OutOfRange)?,
                };
                w.write_i8(t).or(Err(SerializeError::FailedToWrite))?;
                w.append(&mut v);
                Ok(w)
            },
            Value::Timestamp(v) => {
                if v.timestamp() >> 34 == 0 {
                    let value = (u64::from(v.timestamp_subsec_nanos()) << 34) | (v.timestamp() as u64);
                    if value & 0xffff_ffff_0000_0000 == 0 {
                        let mut w = Vec::with_capacity(1 + 1 + 4);
                        w.write_u8(Marker::FixExt4.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_i8(-1).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u32::<BigEndian>(value as u32).or(Err(SerializeError::FailedToWrite))?;
                        Ok(w)
                    } else {
                        let mut w = Vec::with_capacity(1 + 1 + 8);
                        w.write_u8(Marker::FixExt8.into()).or(Err(SerializeError::FailedToWrite))?;
                        w.write_i8(-1).or(Err(SerializeError::FailedToWrite))?;
                        w.write_u64::<BigEndian>(value).or(Err(SerializeError::FailedToWrite))?;
                        Ok(w)
                    }
                } else {
                    let mut w = Vec::with_capacity(1 + 1 + 1 + 4 + 8);
                    w.write_u8(Marker::Ext8.into()).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u8(12).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i8(-1).or(Err(SerializeError::FailedToWrite))?;
                    w.write_u32::<BigEndian>(v.timestamp_subsec_nanos() as u32).or(Err(SerializeError::FailedToWrite))?;
                    w.write_i64::<BigEndian>(v.timestamp()).or(Err(SerializeError::FailedToWrite))?;
                    Ok(w)
                }
            },
        }
    }

    pub fn deserialize<R: Read>(buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        match Marker::from(buf_reader.read_u8().or(Err(DeserializeError::InvalidMarker))?) {
            Marker::PositiveFixInt(n) => Ok(Value::UInt8(n)),
            Marker::FixArray(n) => Self::deserialize_array(n as usize, buf_reader),
            Marker::FixStr(n) => Self::deserialize_string(n as usize, buf_reader),
            Marker::Nil => Ok(Value::Nil),
            Marker::True => Ok(Value::Bool(true)),
            Marker::False => Ok(Value::Bool(false)),
            Marker::Bin8 => Self::deserialize_binary(buf_reader.read_u8().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Bin16 => Self::deserialize_binary(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Bin32 => Self::deserialize_binary(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Float32 => Ok(Value::Float32(buf_reader.read_f32::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Float64 => Ok(Value::Float64(buf_reader.read_f64::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::UInt8 => Ok(Value::UInt8(buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?)),
            Marker::UInt16 => Ok(Value::UInt16(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::UInt32 => Ok(Value::UInt32(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::UInt64 => Ok(Value::UInt64(buf_reader.read_u64::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Int8 => Ok(Value::Int8(buf_reader.read_i8().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Int16 => Ok(Value::Int16(buf_reader.read_i16::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Int32 => Ok(Value::Int32(buf_reader.read_i32::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Int64 => Ok(Value::Int64(buf_reader.read_i64::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Str8 => Self::deserialize_string(buf_reader.read_u8().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Str16 => Self::deserialize_string(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Str32 => Self::deserialize_string(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::NegativeFixInt(n) => Ok(Value::Int8(n)),
            _ => unimplemented!(),
        }
    }

    fn deserialize_binary<R: Read>(size: usize, buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        let mut buf = Vec::with_capacity(size);
        unsafe { buf.set_len(size); }
        buf_reader.read_exact(&mut buf[..]).or(Err(DeserializeError::InvalidValue))?;
        Ok(Value::Binary(Binary(buf)))
    }

    fn deserialize_string<R: Read>(size: usize, buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        let mut buf = Vec::with_capacity(size);
        unsafe { buf.set_len(size); }
        buf_reader.read_exact(&mut buf).or(Err(DeserializeError::InvalidValue))?;
        Ok(Value::String(String::from_utf8(buf).or(Err(DeserializeError::InvalidValue))?))
    }

    fn deserialize_array<R: Read>(size: usize, buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        let mut buf = Vec::with_capacity(size);
        for _ in 0..size {
            buf.push(Self::deserialize(buf_reader)?);
        }
        Ok(Value::Array(buf))
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        value.map_or(Value::Nil, Into::into)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float64(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::UInt8(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::UInt16(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::UInt32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::UInt64(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Int8(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Int16(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int64(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<Binary> for Value {
    fn from(value: Binary) -> Self {
        Value::Binary(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(String::from(value))
    }
}

impl From<Vec<Self>> for Value {
    fn from(value: Vec<Self>) -> Self {
        Value::Array(value)
    }
}

impl From<&[Self]> for Value {
    fn from(value: &[Self]) -> Self {
        Value::Array(Vec::from(value))
    }
}

impl From<BTreeMap<String, Self>> for Value {
    fn from(value: BTreeMap<String, Self>) -> Self {
        Value::Map(value)
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(value: DateTime<Utc>) -> Self {
        Value::Timestamp(value)
    }
}
