use crate::deserializable::Deserializable;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

pub struct Deserializer<R: Read + Seek>(BufReader<R>);

#[derive(Debug)]
pub enum Error {
    FailedToFillBuf,
    FailedToDeserialize(u64),
    FailedToSeek,
}

impl<R: Read + Seek> Deserializer<R> {
    pub fn new(buf_reader: BufReader<R>) -> Self {
        Deserializer(buf_reader)
    }

    pub fn deserialize<T: Deserializable, F: FnMut(T, u64) -> ()>(mut self, mut f: F) -> Result<(), Error> {
        while !self.0.fill_buf().or(Err(Error::FailedToFillBuf))?.is_empty() {
            let position = self.0.seek(SeekFrom::Current(0)).or(Err(Error::FailedToSeek))?;
            T::deserialize(&mut self.0).map(|v| f(v, position)).or(Err(Error::FailedToDeserialize(position)))?;
        }
        Ok(())
    }
}
