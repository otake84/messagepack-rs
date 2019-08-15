use messagepack_rs::deserializable::Deserializable;
use messagepack_rs::serializable::Serializable;
use messagepack_rs::value::Value;
use std::io::{BufReader, Cursor};

#[derive(Clone, Debug, PartialEq)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<Rgba> for Value {
    fn from(value: Rgba) -> Self {
        Self::Extension(0, vec![value.r, value.g, value.b, value.a])
    }
}

fn main() {
    let rgba = Rgba { r: 5, g: 10, b: 15, a: 20 };
    let value = Value::from(rgba);
    println!("{:?}", value);

    let serialized_value = value.serialize().unwrap();
    println!("{:?}", serialized_value);

    let deserialized_value = Value::deserialize(&mut BufReader::new(Cursor::new(serialized_value))).unwrap();
    println!("{:?}", deserialized_value);
}
