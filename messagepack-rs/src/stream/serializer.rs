use crate::serializable::*;
use std::io::{BufWriter, Error, Write};

pub struct Serializer<T: Write>(BufWriter<T>);

impl<W: Write> Serializer<W> {
    pub fn new(w: W) -> Self {
        Serializer(BufWriter::new(w))
    }

    pub fn serialize<T: Serializable>(&mut self, value: T) -> Result<usize, SerializeError> {
        self.0.write(&value.serialize()?).or(Err(SerializeError::FailedToWrite))
    }

    pub fn get_ref(&self) -> &W {
        self.0.get_ref()
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.0.flush()
    }
}

