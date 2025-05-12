use serde::Serialize;
use std::any::Any;

#[derive(Serialize)]
pub struct SerializableError {
    pub message: String,
}

// these implementations of `From` are required
impl From<String> for SerializableError {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}

impl From<Box<dyn Any + Send>> for SerializableError {
    fn from(value: Box<dyn Any + Send>) -> Self {
        let message = value
            .downcast_ref::<&'static str>()
            .map(|c| String::from(*c))
            .or_else(|| value.downcast_ref::<String>().cloned())
            .unwrap_or_default();
        Self { message }
    }
}

impl From<bincode::error::DecodeError> for SerializableError {
    fn from(value: bincode::error::DecodeError) -> Self {
        Self {
            message: format!("Bincode Decode Error: {value}"),
        }
    }
}

impl From<bincode::error::EncodeError> for SerializableError {
    fn from(value: bincode::error::EncodeError) -> Self {
        Self {
            message: format!("Bincode Encode Error: {value}"),
        }
    }
}
