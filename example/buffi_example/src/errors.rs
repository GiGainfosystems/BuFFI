use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Serialize, Deserialize)]
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

impl From<Box<bincode::ErrorKind>> for SerializableError {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        Self {
            message: format!("Bincode: {value}"),
        }
    }
}
