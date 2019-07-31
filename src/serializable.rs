#[derive(Debug)]
pub enum SerializeError {
    FailedToWrite,
    OutOfRange,
}

pub trait Serializable {
    fn serialize(self) -> Result<Vec<u8>, SerializeError>;
}
