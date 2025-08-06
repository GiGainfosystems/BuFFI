#![allow(unexpected_cfgs)]

use cgmath::Point1;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// A TestClient that you might use to hold a database connection
pub struct TestClient {
    runtime: Arc<Runtime>,
}

/// A function that is not part of an impl block
#[buffi::exported]
pub fn free_standing_function(input: i64) -> Result<i64, String> {
    Ok(input)
}

/// Get a client to call functions
#[unsafe(no_mangle)]
pub extern "C" fn get_test_client() -> *mut TestClient {
    let client = TestClient {
        runtime: Arc::new(Runtime::new().unwrap()),
    };
    Box::leak(Box::new(client))
}

/// A custom type that needs to be available in C++ as well
#[derive(Serialize)]
pub struct CustomType {
    /// Some content
    pub some_content: i64,
    /// A cyclic reference that's a bit more complex
    pub itself: Option<Box<CustomType>>,
    /// An enum that contains a remote type that we would like to use in the API
    pub random_enum: RandomEnum,
}

#[derive(Serialize)]
pub enum RandomEnum {
    /// An empty case that is here to make the test simpler
    NoValue,
    /// A timestamp from chrono that we would like to use in the API
    TimeStamp(#[serde(with = "crate::DateTimeHelper")] DateTime<Utc>),
}

#[derive(Serialize)]
#[serde(remote = "DateTime<Utc>")]
pub struct DateTimeHelper {
    /// milliseconds since 1.1.1970 00:00:00
    #[serde(getter = "DateTime::timestamp_millis")]
    pub milliseconds_since_unix_epoch: i64,
}

impl From<DateTimeHelper> for DateTime<Utc> {
    fn from(value: DateTimeHelper) -> Self {
        DateTime::from_timestamp_millis(value.milliseconds_since_unix_epoch)
            .expect("Valid timestamp")
    }
}

#[buffi::exported]
impl TestClient {
    /// A function that might use context provided by a TestClient to do its thing
    pub fn client_function(&self, input: String) -> Result<String, String> {
        Ok(input)
    }

    /// An async function that needs a `Runtime` to be executed and returns a more complex type
    pub async fn async_function(&self, content: i64) -> Result<CustomType, String> {
        Ok(CustomType {
            some_content: content,
            itself: None,
            random_enum: RandomEnum::NoValue,
        })
    }

    /// Here we use a type from a third party crate and return `()`
    pub fn use_foreign_type_and_return_nothing(&self, point: Point1<f64>) -> Result<(), String> {
        println!("{point:?}");
        Ok(())
    }
}

/// This function frees a byte buffer allocated on the Rust side
///
/// * `ptr`: The ptr to the buffer
/// * `size`: The size of the buffer
///
/// # Safety
///
/// Calling this function outside a destructor is highly unsafe
/// and result in a use-after-free
#[unsafe(no_mangle)]
pub unsafe extern "C" fn buffi_free_byte_buffer(ptr: *mut u8, size: usize) {
    if !ptr.is_null() {
        // SAFETY: We checked for null above
        let v = unsafe { Vec::from_raw_parts(ptr, size, size) };
        drop(v);
    }
}

pub mod errors;
