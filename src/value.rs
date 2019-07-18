use std::convert::From;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}
