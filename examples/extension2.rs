use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use messagepack_rs::deserializable::{Deserializable, DeserializeError};
use messagepack_rs::marker::Marker;
use messagepack_rs::serializable::{Serializable, SerializeError};
use messagepack_rs::value::Value;
use std::collections::BTreeMap;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};

#[derive(Clone, Debug, PartialEq)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Clone, Debug, PartialEq)]
enum MyValue {
    Value(Value),
    Array(Vec<MyValue>),
    Map(BTreeMap<String, MyValue>),
    Rgba(Rgba),
}

impl From<Value> for MyValue {
    fn from(value: Value) -> Self {
        MyValue::Value(value)
    }
}

impl From<Rgba> for MyValue {
    fn from(value: Rgba) -> Self {
        MyValue::Rgba(value)
    }
}

impl From<Vec<MyValue>> for MyValue {
    fn from(value: Vec<MyValue>) -> Self {
        MyValue::Array(value)
    }
}

impl From<BTreeMap<String, MyValue>> for MyValue {
    fn from(value: BTreeMap<String, MyValue>) -> Self {
        MyValue::Map(value)
    }
}

impl Serializable for MyValue {
    fn serialize(self) -> Result<Vec<u8>, SerializeError> {
        match self {
            MyValue::Value(v) => v.serialize(),
            MyValue::Array(v) => Self::serialize_array(v),
            MyValue::Map(v) => Self::serialize_map(v),
            MyValue::Rgba(v) => {
                let mut w = Vec::with_capacity(1 + 1 + 4);
                w.write_u8(Marker::FixExt4.into()).or(Err(SerializeError::FailedToWrite))?;
                w.write_i8(0).or(Err(SerializeError::FailedToWrite))?;
                w.write_u8(v.r).or(Err(SerializeError::FailedToWrite))?;
                w.write_u8(v.g).or(Err(SerializeError::FailedToWrite))?;
                w.write_u8(v.b).or(Err(SerializeError::FailedToWrite))?;
                w.write_u8(v.a).or(Err(SerializeError::FailedToWrite))?;
                Ok(w)
            },
        }
    }
}

impl Deserializable for MyValue {
    fn deserialize<R: Read + Seek>(buf_reader: &mut BufReader<R>) -> Result<Self, DeserializeError> {
        match Marker::from(buf_reader.read_u8().or(Err(DeserializeError::InvalidMarker))?) {
            Marker::FixArray(n) => Self::deserialize_array(n as usize, buf_reader),
            Marker::Array16 => Self::deserialize_array(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Array32 => Self::deserialize_array(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::FixMap(n) => Self::deserialize_map(n as usize, buf_reader),
            Marker::Map16 => Self::deserialize_map(buf_reader.read_u16::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::Map32 => Self::deserialize_map(buf_reader.read_u32::<BigEndian>().or(Err(DeserializeError::InvalidLength))? as usize, buf_reader),
            Marker::FixExt4 => {
                let t = buf_reader.read_u8().or(Err(DeserializeError::InvalidMarker))?;
                if t == 0 {
                    Ok(From::from(Rgba {
                        r: buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?,
                        g: buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?,
                        b: buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?,
                        a: buf_reader.read_u8().or(Err(DeserializeError::InvalidValue))?,
                    }))
                } else {
                    buf_reader.seek(SeekFrom::Current(-2)).or(Err(DeserializeError::InvalidMarker))?;
                    Ok(MyValue::from(Value::deserialize(buf_reader)?))
                }
            },
            _ => {
                buf_reader.seek(SeekFrom::Current(-1)).or(Err(DeserializeError::InvalidMarker))?;
                Ok(MyValue::from(Value::deserialize(buf_reader)?))
            }
        }
    }
}

fn main() {
    let rgba = Rgba { r: 5, g: 10, b: 15, a: 20 };
    let value = MyValue::from(rgba);
    println!("{:?}", value);

    let serialized_value = value.serialize().unwrap();
    println!("{:?}", serialized_value);

    let deserialized_value = MyValue::deserialize(&mut BufReader::new(Cursor::new(serialized_value))).unwrap();
    println!("{:?}", deserialized_value);
}
