use syn::Ident;

// This code is shared with buffi itself to reuse the attribute parsing

#[derive(Debug)]
#[allow( // allow and not expect as it's used in one of the locations
    dead_code,
    reason = "This exists for verifying that the attributes are sane"
)]
pub enum BuffiAnnotation {
    Tpe(syn::Path),
    Skip,
}

impl syn::parse::Parse for BuffiAnnotation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::ext::IdentExt;

        let ident = Ident::parse_any(input)?;
        match &ident {
            i if i == "type" => {
                let _ = input.parse::<syn::Token![=]>()?;
                let path = input.parse()?;
                Ok(Self::Tpe(path))
            }
            i if i == "skip" => {
                if input.is_empty() {
                    Ok(Self::Skip)
                } else {
                    Err(syn::Error::new(input.span(), "unexpected tokens"))
                }
            }
            i => Err(syn::Error::new(
                ident.span(),
                format!("Expected `type`, but got `{i}`"),
            )),
        }
    }
}
