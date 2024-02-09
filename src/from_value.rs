use crate::Value;

pub trait FromValue: Sized {
    fn from_value(value: &Value) -> Option<Self>;
}

impl FromValue for Value {
    fn from_value(value: &Value) -> Option<Self> {
        Some(value.clone())
    }
}

impl FromValue for i32 {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::I32(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromValue for String {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(value) => Some(value.clone()),
            _ => None,
        }
    }
}

pub struct StringFirstChar(pub char);

impl FromValue for StringFirstChar {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => s.chars().next().map(Self),
            _ => None,
        }
    }
}

pub struct StringLastChar(pub char);

impl FromValue for StringLastChar {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => s.chars().next_back().map(Self),
            _ => None,
        }
    }
}
