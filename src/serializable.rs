use byteorder::{BigEndian, WriteBytesExt};
use crate::marker::Marker;

#[derive(Debug)]
pub enum SerializeError {
    FailedToWrite,
    OutOfRange,
}

pub trait Serializable: Sized {
    fn serialize(self) -> Result<Vec<u8>, SerializeError>;

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
}
