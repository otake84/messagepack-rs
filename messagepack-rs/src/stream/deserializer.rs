use crate::deserializable::Deserializable;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::marker::PhantomData;

pub struct Deserializer<D: Deserializable, R: Read + Seek> {
    buf_reader: BufReader<R>,
    phantom: PhantomData<D>,
}

#[derive(Debug)]
pub enum Error {
    FailedToDeserialize(u64),
    FailedToFillBuf,
    FailedToSeek,
}

impl<D: Deserializable, R: Read + Seek> Deserializer<D, R> {
    pub fn new(buf_reader: BufReader<R>) -> Self {
        Deserializer { buf_reader, phantom: PhantomData::<D> }
    }
}

impl<D: Deserializable, R: Read + Seek> Iterator for Deserializer<D, R> {
    type Item = Result<(D, u64), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf_reader.fill_buf() {
            Ok(result) => {
                if !result.is_empty() {
                    match self.buf_reader.seek(SeekFrom::Current(0)) {
                        Ok(position) => {
                            match D::deserialize(&mut self.buf_reader) {
                                Ok(v) => Some(Ok((v, position))),
                                _ => Some(Err(Error::FailedToDeserialize(position)))
                            }
                        },
                        _ => Some(Err(Error::FailedToSeek))
                    }
                } else {
                    None
                }
            },
            _ => Some(Err(Error::FailedToFillBuf)),
        }
    }
}
