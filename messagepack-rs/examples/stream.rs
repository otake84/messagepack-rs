use messagepack_rs::stream::serializer::Serializer;
use messagepack_rs::value::Value;
use std::io::{BufReader, Cursor};

fn main() {
    let buf = Vec::new();
    let mut stream_serializer = Serializer::new(buf);
    stream_serializer.serialize(Value::Nil).unwrap();
    stream_serializer.serialize(Value::from(true)).unwrap();
    stream_serializer.serialize(Value::from(false)).unwrap();
    stream_serializer.serialize(Value::from("test")).unwrap();
    stream_serializer.serialize(Value::Nil).unwrap();
    stream_serializer.flush().unwrap();
    println!("{:?}", stream_serializer.get_ref());

    let buf_reader = BufReader::new(Cursor::new(stream_serializer.get_ref()));
    let stream_deserializer = messagepack_rs::stream::deserializer::Deserializer::<Value, _>::new(buf_reader);
    stream_deserializer.for_each(|v| println!("{:?}", v));
}
