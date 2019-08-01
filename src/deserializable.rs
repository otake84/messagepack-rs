use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength,
    InvalidMarker,
    InvalidValue,
}

pub trait Deserializable: Sized + From<Vec<Self>> {
    fn deserialize<R: Read + Seek>(buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError>;

    fn deserialize_array<R: Read + Seek>(size: usize, buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        let mut buf = Vec::with_capacity(size);
        for _ in 0..size {
            buf.push(Self::deserialize(buf_reader)?);
        }
        Ok(From::from(buf))
    }
}
