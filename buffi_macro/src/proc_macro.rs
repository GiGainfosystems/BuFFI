// Copyright (C) 2023 by GiGa infosystems
//! This file contains the implementation of the `#[buffi_macro::exported]` attribute macro
use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::FUNCTION_PREFIX;

// the prefix parameter is here in preparation for whenever we want to customize that as well
pub(crate) fn expand(
    impl_item: syn::Item,
    prefix: Option<String>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let prefix = prefix.unwrap_or_else(|| FUNCTION_PREFIX.to_string());
    let mut exports = Vec::new();
    if cfg!(feature = "with_c_api") {
        if let syn::Item::Impl(ref impl_item) = impl_item {
            generate_exported_functions_for_impl_block(impl_item, &mut exports, prefix)?;
        } else if let syn::Item::Fn(ref fn_item) = impl_item {
            let docs = fn_item.attrs.iter().filter(|a| a.path().is_ident("doc"));
            generate_exported_function(
                &fn_item.sig,
                Vec::new(),
                &mut exports,
                docs,
                fn_item.span(),
                prefix,
            )?;
        } else {
            panic!("Unknown")
        };
    }
    Ok(quote::quote! {
        #[cfg(not(generated_extern_impl))]
        #impl_item

        #(#exports)*
    })
}

fn generate_exported_functions_for_impl_block(
    impl_item: &syn::ItemImpl,
    exports: &mut Vec<proc_macro2::TokenStream>,
    prefix: String,
) -> Result<(), syn::Error> {
    let mut syn_error: Option<syn::Error> = None;
    for item in &impl_item.items {
        if let syn::ImplItem::Fn(m) = item {
            if matches!(m.vis, syn::Visibility::Public(_)) {
                let self_ty = &impl_item.self_ty;
                let docs = m.attrs.iter().filter(|a| a.path().is_ident("doc"));

                let mut arg_list = Vec::new();
                arg_list.push(quote::quote!(this_ptr: *mut #self_ty));

                match generate_exported_function(
                    &m.sig,
                    arg_list,
                    exports,
                    docs,
                    item.span(),
                    prefix.clone(),
                ) {
                    Ok(_) => (),
                    Err(new_error) => {
                        if let Some(e) = syn_error.as_mut() {
                            e.combine(new_error);
                        } else {
                            syn_error = Some(new_error);
                        }
                    }
                }
            }
        }
    }

    if let Some(e) = syn_error {
        Err(e)
    } else {
        Ok(())
    }
}

fn generate_exported_function<'a>(
    sig: &syn::Signature,
    mut arg_list: Vec<proc_macro2::TokenStream>,
    exports: &mut Vec<proc_macro2::TokenStream>,
    docs: impl Iterator<Item = &'a syn::Attribute>,
    item_span: Span,
    prefix: String,
) -> Result<(), syn::Error> {
    let is_result_type = match &sig.output {
        syn::ReturnType::Type(_, boxed_type) => {
            if let syn::Type::Path(type_path) = &**boxed_type {
                type_path
                    .path
                    .segments
                    .last()
                    .expect("type path should have at least one segment")
                    .ident
                    == "Result"
            } else {
                false
            }
        }
        _ => false,
    };
    if !is_result_type {
        let func_name = &sig.ident;
        let func_span = sig.output.span();
        return Err(syn::Error::new(
            func_span,
            format!("API function '{func_name}' is not returning a 'Result'"),
        ));
    }

    let is_free_standing = arg_list.is_empty();
    let name = &sig.ident;
    let fn_name = syn::Ident::new(&format!("{}_{}", prefix, sig.ident), sig.ident.span());
    for arg in &sig.inputs {
        if let syn::FnArg::Typed(t) = arg {
            let n = if let syn::Pat::Ident(ref i) = *t.pat {
                i.ident.clone()
            } else {
                panic!("unknown")
            };
            let n_size = syn::Ident::new(&format!("{n}_size"), n.span());
            arg_list.push(quote::quote!(#n: *const u8));
            arg_list.push(quote::quote!(#n_size: usize));
        }
    }
    arg_list.push(quote::quote!(out_ptr: *mut *mut u8));
    let deserialized_args = sig.inputs.iter().filter_map(|arg| {
        let span = arg.span();
        if let syn::FnArg::Typed(t) = arg {
            let n = if let syn::Pat::Ident(ref i) = *t.pat {
                i.ident.clone()
            } else {
                panic!("unknown")
            };
            let n_size = syn::Ident::new(&format!("{n}_size"), n.span());
            Some(quote::quote_spanned! {span=>
                let slice = if #n.is_null() {
                    &[]
                } else {
                    unsafe {
                        std::slice::from_raw_parts(#n, #n_size)
                    }
                };
                let #n = bincode::deserialize(slice)?;
            })
        } else {
            None
        }
    });
    let args = sig.inputs.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(t) = arg {
            let n = if let syn::Pat::Ident(ref i) = *t.pat {
                i.ident.clone()
            } else {
                panic!("unknown")
            };
            Some(n)
        } else {
            None
        }
    });
    let mut_this = sig.inputs.first().and_then(|s| {
        if let syn::FnArg::Receiver(r) = s {
            r.mutability.map(|_| quote::quote!(mut))
        } else {
            None
        }
    });
    let await_call = if sig.asyncness.is_some() {
        Some(quote::quote!(.await))
    } else {
        None
    };
    let map_err_call = if let syn::ReturnType::Type(_, ref tpe) = sig.output {
        if let syn::Type::Path(p) = &**tpe {
            if p.path
                .segments
                .last()
                .map(|s| s.ident == "Result")
                .unwrap_or(false)
            {
                Some(quote::quote!(.map_err(crate::errors::SerializableError::from)))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let (
        mut tracing_pointer,
        mut tracing_out_pointer,
        mut tracing_skip,
        mut tracing_error,
        mut tracing_serializable_e,
        mut tracing_serializable_w,
        mut allow_unwrap_default,
    ) = Default::default();

    if cfg!(feature = "with_tracing") {
        tracing_pointer = Some(quote::quote! {tracing::error!("This pointer is null");});
        tracing_out_pointer = Some(quote::quote! {tracing::error!("Out pointer is null");});
        tracing_skip = Some(quote::quote! {#[tracing::instrument(skip_all)]});
        tracing_error = Some(quote::quote! {tracing::error!("Error");});
        tracing_serializable_e = Some(quote::quote! {tracing::error!(%_e, "Serialization error");});
        tracing_serializable_w = Some(quote::quote! {tracing::warn!(%e, "Serialization error");});
    } else {
        allow_unwrap_default = Some(quote::quote! {#[allow(clippy::manual_unwrap_or_default)]});
    }

    let this_ptr = if is_free_standing {
        None
    } else {
        Some(quote::quote_spanned! {item_span=>
            if this_ptr.is_null() {
                #tracing_pointer
                return Err(color_eyre::eyre::eyre!("This pointer is null").into());
            }
            let this = unsafe { &#mut_this *this_ptr };
        })
    };
    let out_ptr = quote::quote_spanned! {item_span=>
        if out_ptr.is_null() {
            #tracing_out_pointer
            return Err(color_eyre::eyre::eyre!("Out pointer is null").into());
        }
    };
    let deserialize = quote::quote! {
        #this_ptr
        #out_ptr
        #(#deserialized_args)*
    };
    let inner_block = if is_free_standing {
        quote::quote! {
            #name(#(#args,)*)#await_call #map_err_call
        }
    } else {
        quote::quote_spanned! {item_span=>
            this.#name(#(#args,)*)#await_call #map_err_call
        }
    };
    let inner_block = if sig.asyncness.is_some() {
        quote::quote! {
            #deserialize
            let runtime = std::sync::Arc::clone(&this.runtime);
            let fut = async move {
                #inner_block
            };
            runtime.block_on(fut)
        }
    } else {
        quote::quote! {
            #deserialize
            #inner_block
        }
    };
    exports.push(quote::quote_spanned! {item_span=>
        #(#docs)*
        ///
        /// # Safety
        /// Unsafe code is used to check input and output pointers to byte buffers.
        #[cfg(not(generated_extern_function_marker))]
        #tracing_skip
        #allow_unwrap_default
        #[no_mangle]
        pub unsafe extern "C" fn #fn_name(#(#arg_list,)*) -> usize {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #inner_block
            }));

            let mut res = match r {
                Ok(o) => {
                    o
                },
                Err(e) => {
                    #tracing_error
                    Err(crate::errors::SerializableError::from(e))
                }
            };
            let bytes = match bincode::serialize(&res) {
                Ok(bytes) => {
                    bytes
                }
                Err(e) => {
                    #tracing_serializable_w
                    res = Err(e.into());
                    match bincode::serialize(&res) {
                        Ok(bytes) => {
                            bytes
                        }
                        Err(_e) => {
                            #tracing_serializable_e
                            Vec::new()
                        }
                    }
                }
            };

            let bytes = bytes.into_boxed_slice();
            let len = bytes.len();
            let out: &mut *mut u8 = unsafe { &mut *out_ptr };
            *out = Box::into_raw(bytes) as *mut u8;
            len
        }
    });

    Ok(())
}
