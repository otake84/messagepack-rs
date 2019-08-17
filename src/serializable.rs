use byteorder::{BigEndian, WriteBytesExt};
use chrono::prelude::*;
use crate::binary::Binary;
use crate::extension::Extension;
use crate::marker::Marker;
use std::collections::BTreeMap;
use std::io::Write;

#[derive(Debug)]
pub enum SerializeError {
    FailedToWrite,
    OutOfRange,
}

pub trait Serializable: Sized {
    fn serialize(self) -> Result<Vec<u8>, SerializeError>;

    fn serialize_nil() -> Result<Vec<u8>, SerializeError> {
        Ok(vec![Marker::Nil.into()])
    }

    fn serialize_bool(v: bool) -> Result<Vec<u8>, SerializeError> {
        Ok(if v { vec![Marker::True.into()] } else { vec![Marker::False.into()] })
    }

    fn serialize_float32(v: f32) -> Result<Vec<u8>, SerializeError> {
        let mut w = Vec::with_capacity(1 + 4);
        w.write_u8(Marker::Float32.into()).or(Err(SerializeError::FailedToWrite))?;
        w.write_f32::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
        Ok(w)
    }

    fn serialize_float64(v: f64) -> Result<Vec<u8>, SerializeError> {
        let mut w = Vec::with_capacity(1 + 8);
        w.write_u8(Marker::Float64.into()).or(Err(SerializeError::FailedToWrite))?;
        w.write_f64::<BigEndian>(v).or(Err(SerializeError::FailedToWrite))?;
        Ok(w)
    }

    fn serialize_uint8(v: u8) -> Result<Vec<u8>, SerializeError> {
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
    }

    fn serialize_uint16(v: u16) -> Result<Vec<u8>, SerializeError> {
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
    }

    fn serialize_uint32(v: u32) -> Result<Vec<u8>, SerializeError> {
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
    }

    fn serialize_uint64(v: u64) -> Result<Vec<u8>, SerializeError> {
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
    }

    fn serialize_int8(v: i8) -> Result<Vec<u8>, SerializeError> {
        if v >= -0b0010_0000 {
            let mut w = Vec::with_capacity(1);
            w.write_i8(v).or(Err(SerializeError::FailedToWrite))?;
            Ok(w)
        } else {
            let mut w = Vec::with_capacity(1 + 1);
            w.write_u8(Marker::Int8.into()).or(Err(SerializeError::FailedToWrite))?;
            w.write_i8(v).or(Err(SerializeError::FailedToWrite))?;
            Ok(w)
        }
    }

    fn serialize_int16(v: i16) -> Result<Vec<u8>, SerializeError> {
        if v >= -0b0010_0000 {
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
    }

    fn serialize_int32(v: i32) -> Result<Vec<u8>, SerializeError> {
        if v >= -0b0010_0000 {
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
    }

    fn serialize_int64(v: i64) -> Result<Vec<u8>, SerializeError> {
        if v >= -0b0010_0000 {
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
    }

    fn serialize_binary(v: Binary) -> Result<Vec<u8>, SerializeError> {
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
    }

    fn serialize_string(v: String) -> Result<Vec<u8>, SerializeError> {
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
    }

    fn serialize_array(v: Vec<Self>) -> Result<Vec<u8>, SerializeError> {
        let mut values = Vec::new();
        let len = v.len();
        for vv in v.into_iter() {
            values.append(&mut Self::serialize(vv)?);
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
    }

    fn serialize_map(v: BTreeMap<String, Self>) -> Result<Vec<u8>, SerializeError> {
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
            w.write_all(&Self::serialize_string(k)?).or(Err(SerializeError::FailedToWrite))?;
            w.write_all(&v.serialize()?).or(Err(SerializeError::FailedToWrite))?;
        }
        Ok(w)
    }

    fn serialize_extension(mut v: Extension) -> Result<Vec<u8>, SerializeError> {
        let mut w = match v.data.len() {
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
        w.write_i8(v.t).or(Err(SerializeError::FailedToWrite))?;
        w.append(&mut v.data);
        Ok(w)
    }

    fn serialize_timestamp(v: DateTime<Utc>) -> Result<Vec<u8>, SerializeError> {
        if v.timestamp() >> 34 == 0 {
            let value = (u64::from(v.timestamp_subsec_nanos()) << 34) | (v.timestamp() as u64);
            if value & 0xff_ff_ff_ff_00_00_00_00 == 0 {
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
    }
}
