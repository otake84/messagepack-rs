use byteorder::{BigEndian, ReadBytesExt};
use crate::marker::Marker;
use std::collections::BTreeMap;
use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength,
    InvalidMarker,
    InvalidValue,
}

pub trait Deserializable: Sized + From<Vec<Self>> + From<BTreeMap<String, Self>> {
    fn deserialize<R: Read + Seek>(buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError>;

    fn deserialize_array<R: Read + Seek>(size: usize, buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        let mut buf = Vec::with_capacity(size);
        for _ in 0..size {
            buf.push(Self::deserialize(buf_reader)?);
        }
        Ok(From::from(buf))
    }

    fn deserialize_map<R: Read + Seek>(size: usize, buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        fn deserialize_string_primitive<R: Read>(buf_reader: &mut BufReader<R>) -> Result<String, DeserializeError> {
            let mut buf = match From::from(buf_reader.read_u8().or(Err(DeserializeError::InvalidMarker))?) {
                Marker::FixStr(n) => Vec::with_capacity(n as usize),
                Marker::Str8 => Vec::with_capacity(buf_reader.read_u8().or(Err(DeserializeError::InvalidLength))? as usize),
                Marker::Str16 => Vec::with_capacity(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize),
                Marker::Str32 => Vec::with_capacity(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize),
                _ => Err(DeserializeError::InvalidMarker)?
            };
            unsafe { buf.set_len(buf.capacity()); }
            buf_reader.read_exact(&mut buf[..]).or(Err(DeserializeError::InvalidValue))?;
            String::from_utf8(buf).or(Err(DeserializeError::InvalidValue))
        }

        let mut buf = BTreeMap::new();
        for _ in 0..size {
            buf.insert(deserialize_string_primitive(buf_reader)?, Self::deserialize(buf_reader)?);
        }
        Ok(From::from(buf))
    }
}
