use proc_macro::TokenStream;
use prockit::DeriveImpls as _;

use crate::fvia::FromVia;

mod fvia;
mod target;

#[proc_macro_derive(From, attributes(from_via))]
pub fn derive_from_via(input: TokenStream) -> TokenStream {
    FromVia::derive_impls(input)
}
