use proc_macro::TokenStream;
use prockit::ProcMacroDeriveBase as _;

use crate::fvia::FromVia;

mod fvia;
mod target;

#[proc_macro_derive(From, attributes(from_via))]
pub fn derive_from_via(input: TokenStream) -> TokenStream {
    FromVia::proc_macro_derive_base(input)
}
