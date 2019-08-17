use chrono::prelude::*;
use messagepack_rs::deserializable::Deserializable;
use messagepack_rs::serializable::Serializable;
use messagepack_rs::value::Value;
use std::io::{BufReader, Cursor};

fn main() {
    let value = Value::from(vec![Value::from(Utc::now()), Value::from(123u8), Value::from("test"), Value::from(true), Value::Nil, Value::from(None::<u8>)]);
    println!("{:?}", value);
    let serialized_value = value.serialize().unwrap();
    println!("{:?}", serialized_value);
    let deserialized_value = Value::deserialize(&mut BufReader::new(Cursor::new(serialized_value))).unwrap();
    println!("{:?}", deserialized_value);
}
