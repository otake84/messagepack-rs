use std::io::{BufReader, Read};

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength,
    InvalidMarker,
    InvalidValue,
}

pub trait Deserializable: Sized {
    fn deserialize<R: Read>(buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError>;
}
