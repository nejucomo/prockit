use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;

pub struct ParsedDef {}

impl Parse for ParsedDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input;
        todo!()
    }
}

impl ToTokens for ParsedDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ = tokens;
        todo!()
    }
}
