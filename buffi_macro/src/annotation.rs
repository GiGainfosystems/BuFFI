use proc_macro2::TokenStream;
use syn::DeriveInput;
use syn::spanned::Spanned;

use crate::buffi_annotation_attributes::BuffiAnnotation;

pub fn expand(item: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut verifier = VerifyAttributes {
        errors: Vec::new(),
        tps: Vec::new(),
    };

    syn::visit::visit_derive_input(&mut verifier, &item);

    if let Some(mut err) = verifier.errors.pop() {
        for e in verifier.errors {
            err.combine(e);
        }
        Err(err)
    } else if verifier.tps.is_empty() {
        Ok(TokenStream::new())
    } else {
        let bounds = verifier.tps.into_iter().map(
            |VerifyAttribute {
                 span,
                 rust_type,
                 cpp_type,
             }| {
                quote::quote_spanned! {span=> #cpp_type: buffi::SafeTypeMapping<#rust_type>}
            },
        );
        Ok(quote::quote! {
            const _:() = {
                fn __check() where #(#bounds,)* {}
            };
        })
    }
}

#[derive(Debug)]
struct VerifyAttribute {
    span: proc_macro2::Span,
    rust_type: syn::Type,
    cpp_type: syn::Type,
}

#[derive(Debug)]
struct VerifyAttributes {
    errors: Vec<syn::Error>,
    tps: Vec<VerifyAttribute>,
}

impl<'ast> syn::visit::Visit<'ast> for VerifyAttributes {
    fn visit_field(&mut self, f: &'ast syn::Field) {
        let buffi_attrs = f
            .attrs
            .iter()
            .filter_map(|a| {
                if a.path().is_ident("buffi") {
                    Some(a.parse_args::<BuffiAnnotation>())
                } else {
                    None
                }
            })
            .collect::<Result<Vec<_>, _>>();
        let serde_attrs = f
            .attrs
            .iter()
            .filter_map(|a| {
                if a.path().is_ident("serde") {
                    Some(a.parse_args::<SerdeAttr>())
                } else {
                    None
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_default();
        let buffi_attrs = match buffi_attrs {
            Ok(buffi_attrs) => buffi_attrs,
            Err(e) => {
                self.errors.push(e);
                Vec::new()
            }
        };

        let has_skip = buffi_attrs
            .iter()
            .any(|b| matches!(b, BuffiAnnotation::Skip));
        // if we have a buffi skip attribute we need to verify that
        // there is also a `#[serde(default [= whatever])]` attribute
        // otherwise this is an error
        if has_skip
            && !(serde_attrs.contains(&SerdeAttr::Skip)
                && serde_attrs.contains(&SerdeAttr::Default))
        {
            self.errors.push(syn::Error::new(
                f.span(),
                "`#[buffi(skip)]` requires `#[serde(default)]` and `#[serde(skip)]` as well",
            ));
        }
        if let Some(BuffiAnnotation::Tpe(buffi_type)) = buffi_attrs
            .iter()
            .find(|b| matches!(b, BuffiAnnotation::Tpe(_)))
        {
            let buffi_type = buffi_type.clone();
            let deserialize_with = serde_attrs.iter().find_map(|s| match s {
                SerdeAttr::DeserializeWith(p) => Some(p.clone()),
                _ => None,
            });
            let serialize_with = serde_attrs.iter().find_map(|s| match s {
                SerdeAttr::SerializeWith(s) => Some(s.clone()),
                _ => None,
            });
            let mut rust_type = f.ty.clone();
            match (serialize_with, deserialize_with) {
                (Some(mut s), Some(mut d)) => {
                    let _ = s.segments.pop();
                    let last = s.segments.pop().unwrap();
                    s.segments.push_value(last.value().clone());
                    let _ = d.segments.pop();
                    let last = d.segments.pop().unwrap();
                    d.segments.push_value(last.value().clone());
                    if s == d {
                        rust_type = syn::Type::Path(syn::TypePath {
                            qself: None,
                            path: s,
                        });
                    } else {
                        self.errors.push(syn::Error::new(
                            f.ident.span(),
                            "`#[serde(serialize_with)]` needs to refer to the same type as `#[serde(deserialize_with)]`"
                        ));
                    }
                }
                (Some(_), None) | (None, Some(_)) => {
                    self.errors.push(syn::Error::new(
                        f.ident.span(),
                        "`#[buffi(type)]` requires both `#[serde(serialize_with)]` and `#[serde(deserialize_with)]`"
                    ));
                }
                (None, None) => {}
            };
            self.tps.push(VerifyAttribute {
                span: f.ty.span(),
                rust_type,
                cpp_type: syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: buffi_type,
                }),
            });
        }

        syn::visit::visit_field(self, f);
    }
}

#[derive(PartialEq)]
enum SerdeAttr {
    Default,
    Skip,
    SerializeWith(syn::Path),
    DeserializeWith(syn::Path),
    Unknown,
}

impl syn::parse::Parse for SerdeAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        match ident {
            i if i == "default" => Ok(Self::Default),
            i if i == "skip" => Ok(Self::Skip),
            i if i == "deserialize_with" => {
                let _ = input.parse::<syn::Token![=]>()?;
                let path = input.parse::<syn::LitStr>()?.parse::<syn::Path>()?;
                Ok(Self::DeserializeWith(path))
            }
            i if i == "serialize_with" => {
                let _ = input.parse::<syn::Token![=]>()?;
                let path = input.parse::<syn::LitStr>()?.parse::<syn::Path>()?;
                Ok(Self::SerializeWith(path))
            }
            _ => Ok(Self::Unknown),
        }
    }
}
