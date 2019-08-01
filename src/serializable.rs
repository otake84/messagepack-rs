use byteorder::{BigEndian, WriteBytesExt};
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
}
