#![allow(unexpected_cfgs)]

use serde::Serialize;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// A TestClient that you might use to hold a database connection
pub struct TestClient {
    runtime: Arc<Runtime>,
}

/// A function that is not part of an impl block
#[buffi_macro::exported]
pub fn free_standing_function(input: i64) -> Result<i64, String> {
    Ok(input)
}

/// Get a client to call functions
#[no_mangle]
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
}

#[buffi_macro::exported]
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
        })
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
#[no_mangle]
pub unsafe extern "C" fn buffi_free_byte_buffer(ptr: *mut u8, size: usize) {
    if !ptr.is_null() {
        // SAFETY: We checked for null above
        let v = unsafe { Vec::from_raw_parts(ptr, size, size) };
        drop(v);
    }
}

pub mod errors;
