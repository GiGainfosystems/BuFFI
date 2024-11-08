#![allow(unexpected_cfgs)]

/// A TestClient that you might use to hold a database connection
pub struct TestClient {}

/// A function that is not part of an impl block
#[buffi_macro::exported]
pub fn free_standing_function(input: i64) -> Result<i64, String> {
    Ok(input)
}

#[buffi_macro::exported]
impl TestClient {
    /// A function that might use context provided by a TestClient to do its thing
    pub fn client_function(&self, input: String) -> Result<String, String> {
        Ok(input)
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
