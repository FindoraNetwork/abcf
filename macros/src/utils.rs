use quote::ToTokens;
use syn::{Field, parse::Parse};

#[derive(Clone, Debug)]
pub struct ParseField {
    pub inner: Field
}

impl Parse for ParseField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inner = Field::parse_named(input)?;

        Ok(Self {
            inner
        })
    }
}

impl ToTokens for ParseField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.inner.to_tokens(tokens)
    }
}

