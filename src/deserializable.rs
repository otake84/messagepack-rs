use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength,
    InvalidMarker,
    InvalidValue,
}

pub trait Deserializable: Sized {
    fn deserialize<R: Read + Seek>(buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError>;
}
