mod proc_macro;
use ::proc_macro::TokenStream;

const FUNCTION_PREFIX: &str = "buffi";

/// This macro generates a compatible c function for each function in the current impl block
///
/// The generated c function accepts arguments as bincode serialized byte buffers and returns
/// a bincode serialized byte buffer as well. The generated function handles (de)serialization of
/// those buffers internally and converts the arguments/results of each function internally. In
/// addition the generated function contains code to handle panics before reaching the FFI boundary,
/// blocking async funtcions and converting `color_eyre::Report` error types to a FFI compatible version
///
/// The generated c function will be named `buffi_{function_name}`. It accepts a pointer to the current
/// type (`Self`) as first argument. For each other argument of the rust function, two arguments for
/// the c function are generated: `{argument}` as `*const u8` pointing to the serialized argument
/// and `{argument}_size` as `usize` containing a buffer size. In addition a `out_ptr: *mut *mut u8`
/// argument is generated. This pointer will be set to the output buffer allocation. The generated function
/// returns a `usize` indicating the size of the allocated buffer. This buffer needs to be freed
/// via `buffi_free_byte_buffer`
///
/// In addition this macro prepends a `#[tracing::instrument]` attribute to each function
/// in the current impl block
///
/// Modules containing a `#[buffi_macro::exported]` call needs to be public!
#[proc_macro_attribute]
pub fn exported(_att: TokenStream, item: TokenStream) -> TokenStream {
    match syn::parse(item.clone()).and_then(|parsed_item| proc_macro::expand(parsed_item, None)) {
        Ok(tokenstream) => tokenstream,
        Err(e) => {
            let mut out = proc_macro2::TokenStream::from(item);
            out.extend(e.to_compile_error());
            out
        }
    }
    .into()
}
