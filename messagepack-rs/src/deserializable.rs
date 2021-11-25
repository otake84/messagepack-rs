use byteorder::{BigEndian, ReadBytesExt};
use chrono::prelude::*;
use crate::binary::Binary;
use crate::extension::Extension;
use crate::marker::Marker;
use std::collections::BTreeMap;
use std::io::Read;

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength,
    InvalidMarker,
    InvalidValue,
}

pub trait Deserializable: Sized + From<Option<Self>> + From<bool> + From<Binary> + From<f32> + From<f64> + From<u8> + From<u16> + From<u32> + From<u64> + From<i8> + From<i16> + From<i32> + From<i64> + From<String> + From<Vec<Self>> + From<BTreeMap<String, Self>> + From<Extension> + From<DateTime<Utc>> {
    fn deserialize<R: Read>(buf_reader: &mut R) -> Result<Self, DeserializeError> {
        match Marker::from(buf_reader.read_u8().or(Err(DeserializeError::InvalidMarker))?) {
            Marker::PositiveFixInt(n) => Ok(Self::from(n)),
            Marker::FixMap(n) => Self::deserialize_map(n as usize, buf_reader),
            Marker::FixArray(n) => Self::deserialize_array(n as usize, buf_reader),
            Marker::FixStr(n) => Self::deserialize_string(n as usize, buf_reader),
            Marker::Nil => Ok(Self::from(None::<Self>)),
            Marker::Reserved => Err(DeserializeError::InvalidMarker),
            Marker::False => Ok(Self::from(false)),
            Marker::True => Ok(Self::from(true)),
            Marker::Bin8 => Self::deserialize_binary(buf_reader.read_u8().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Bin16 => Self::deserialize_binary(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Bin32 => Self::deserialize_binary(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Ext8 => Self::deserialize_extension(buf_reader.read_u8().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Ext16 => Self::deserialize_extension(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Ext32 => Self::deserialize_extension(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Float32 => Ok(Self::from(buf_reader.read_f32::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Float64 => Ok(Self::from(buf_reader.read_f64::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::UInt8 => Ok(Self::from(buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?)),
            Marker::UInt16 => Ok(Self::from(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::UInt32 => Ok(Self::from(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::UInt64 => Ok(Self::from(buf_reader.read_u64::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Int8 => Ok(Self::from(buf_reader.read_i8().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Int16 => Ok(Self::from(buf_reader.read_i16::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Int32 => Ok(Self::from(buf_reader.read_i32::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::Int64 => Ok(Self::from(buf_reader.read_i64::<BigEndian>().or(Err(DeserializeError::InvalidValue))?)),
            Marker::FixExt1 => Self::deserialize_extension(1, buf_reader),
            Marker::FixExt2 => Self::deserialize_extension(2, buf_reader),
            Marker::FixExt4 => Self::deserialize_extension(4, buf_reader),
            Marker::FixExt8 => Self::deserialize_extension(8, buf_reader),
            Marker::FixExt16 => Self::deserialize_extension(16, buf_reader),
            Marker::Str8 => Self::deserialize_string(buf_reader.read_u8().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Str16 => Self::deserialize_string(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Str32 => Self::deserialize_string(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Array16 => Self::deserialize_array(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Array32 => Self::deserialize_array(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Map16 => Self::deserialize_map(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Map32 => Self::deserialize_map(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::NegativeFixInt(n) => Ok(Self::from(n)),
        }
    }

    fn deserialize_binary<R: Read>(size: usize, buf_reader: &mut R) -> Result<Self, DeserializeError> {
        let mut buf = vec![0; size];
        buf_reader.read_exact(&mut buf[..]).or(Err(DeserializeError::InvalidValue))?;
        Ok(From::from(Binary(buf)))
    }

    fn deserialize_string<R: Read>(size: usize, buf_reader: &mut R) -> Result<Self, DeserializeError> {
        let mut buf = vec![0; size];
        buf_reader.read_exact(&mut buf).or(Err(DeserializeError::InvalidValue))?;
        Ok(From::from(String::from_utf8(buf).or(Err(DeserializeError::InvalidValue))?))
    }

    fn deserialize_array<R: Read>(size: usize, buf_reader: &mut R) -> Result<Self, DeserializeError> {
        let mut buf = Vec::with_capacity(size);
        for _ in 0..size {
            buf.push(Self::deserialize(buf_reader)?);
        }
        Ok(From::from(buf))
    }

    fn deserialize_map<R: Read>(size: usize, buf_reader: &mut R) -> Result<Self, DeserializeError> {
        fn deserialize_string_primitive<R: Read>(buf_reader: &mut R) -> Result<String, DeserializeError> {
            let mut buf = match From::from(buf_reader.read_u8().or(Err(DeserializeError::InvalidMarker))?) {
                Marker::FixStr(n) => vec![0; n as usize],
                Marker::Str8 => vec![0; buf_reader.read_u8().or(Err(DeserializeError::InvalidLength))? as usize],
                Marker::Str16 => vec![0; buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize],
                Marker::Str32 => vec![0; buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize],
                _ => Err(DeserializeError::InvalidMarker)?
            };
            buf_reader.read_exact(&mut buf[..]).or(Err(DeserializeError::InvalidValue))?;
            String::from_utf8(buf).or(Err(DeserializeError::InvalidValue))
        }

        let mut buf = BTreeMap::new();
        for _ in 0..size {
            buf.insert(deserialize_string_primitive(buf_reader)?, Self::deserialize(buf_reader)?);
        }
        Ok(From::from(buf))
    }

    fn deserialize_extension<R: Read>(size: usize, buf_reader: &mut R) -> Result<Self, DeserializeError> {
        let t = buf_reader.read_i8().or(Err(DeserializeError::InvalidLength))?;

        if t == -1 {
            if size == 4 {
                let value = buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidValue))?;
                Utc.timestamp_opt(i64::from(value), 0).single().map(Self::from).ok_or(DeserializeError::InvalidValue)
            } else if size == 8 {
                let value = buf_reader.read_u64::<BigEndian>().or(Err(DeserializeError::InvalidValue))?;
                let nano = value >> 34;
                let sec = value & 0x00_00_00_03_ff_ff_ff_ff;
                Utc.timestamp_opt(sec as i64, nano as u32).single().map(Self::from).ok_or(DeserializeError::InvalidValue)
            } else if size == 12 {
                let nano = buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidValue))?;
                let sec = buf_reader.read_i64::<BigEndian>().or(Err(DeserializeError::InvalidValue))?;
                Utc.timestamp_opt(sec, nano).single().map(Self::from).ok_or(DeserializeError::InvalidValue)
            } else {
                Err(DeserializeError::InvalidValue)
            }
        } else {
            Self::deserialize_extension_for_the_you_type_defined(t, size, buf_reader)
        }
    }

    fn deserialize_extension_for_the_you_type_defined<R: Read>(t: i8, size: usize, buf_reader: &mut R) -> Result<Self, DeserializeError> {
        Self::deserialize_extension_others(t, size, buf_reader)
    }

    fn deserialize_extension_others<R: Read>(t: i8, size: usize, buf_reader: &mut R) -> Result<Self, DeserializeError> {
        let mut data = vec![0; size];
        buf_reader.read_exact(&mut data[..]).or(Err(DeserializeError::InvalidValue))?;
        Ok(From::from(Extension { t, data }))
    }
}
