use ::proc_macro::TokenStream;

mod annotation;
mod buffi_annotation_attributes;
mod proc_macro;

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

/// A helper derive to put annotations for the codegen on struct and enum fields
///
/// Available annotations are `#[buffi(skip)]` and `#[buffi(type = SomeType]`. See below
/// how they can be utilized.
///
/// Putting this derive on a type that is available via a public API allows to modify
/// availability of fields or which specific type should be used in the FFI if type
/// mapping is not obvious.
///
/// Skipping a field
///
/// `#[buffi(skip)]` can be used together with `#[serde(skip)]` and `#[serde(default)]`
/// to hide fields from the FFI. In that case when this type is used in the FFI it will
/// appear on the Rust side with a default value set for that specific field.
///
/// Modifying the type mapping
///
/// `#[buffi(type = SomeType]` can be used in cases where the Rust type is not the desired type for
/// the FFI. It applies either where the binary representation created by serde matches
/// another type (e.g. for `url::Url` and `String` where you have `Url` on the Rust side
/// and want `String` in your FFI) or where the application of custom (de-)serialization
/// is required. This second use case is connected to `#[serde(serialize_with = ...]` and
/// `#[serde(deserialize_with = ...]`. Both annotations have to be present and have to
/// point to functions implemented on a (helper) struct that implements buffi's `SafeTypeMapping`
/// trait.
///
/// Buffi already provides some implementations of `SafeTypeMapping`. Also check if some
/// additional implementations can be utilized via crate features (e.g. `url2`).
#[proc_macro_derive(Annotation, attributes(buffi, serde))]
pub fn annotation(item: TokenStream) -> TokenStream {
    syn::parse(item)
        .and_then(annotation::expand)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
