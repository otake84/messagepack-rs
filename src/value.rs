use std::convert::{From, Into};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Float32(f32),
    Float64(f64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    String(String),
    Array(Vec<Self>),
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        if let Some(value) = value {
            value.into()
        } else {
            Value::Nil
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float64(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::UInt8(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::UInt16(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::UInt32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::UInt64(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Int8(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Int16(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int64(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(String::from(value))
    }
}

impl From<Vec<Self>> for Value {
    fn from(value: Vec<Self>) -> Self {
        Value::Array(value)
    }
}
