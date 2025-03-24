use proc_macro::TokenStream;

mod metapivot;
mod pdef;
mod pm2;
mod simpath;
mod target;
mod typedef;

#[proc_macro_derive(From, attributes(from_via))]
pub fn derive_from_via(input: TokenStream) -> TokenStream {
    pm2::derive_from_via(input.into()).into()
}
