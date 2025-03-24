use proc_macro2::TokenStream;
use quote::ToTokens as _;

use crate::pdef::ParsedDef;

pub fn derive_from_via(input: TokenStream) -> TokenStream {
    derive_from_via_res(input).unwrap_or_else(syn::Error::into_compile_error)
}

pub fn derive_from_via_res(input: TokenStream) -> syn::Result<TokenStream> {
    let pdef: ParsedDef = syn::parse2(input)?;
    Ok(pdef.into_token_stream())
}
