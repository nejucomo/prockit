use proc_macro::TokenStream;

mod pdef;
mod pm2;

#[proc_macro_derive(From, attributes(from_via))]
pub fn derive_from_via(input: TokenStream) -> TokenStream {
    pm2::derive_from_via(input.into()).into()
}
