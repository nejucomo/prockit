use proc_macro2::TokenStream;
use quote::ToTokens;

/// An implementation of a `derive(MyTrait)` proc macro
pub trait ProcMacroDeriveBase: Sized + ToTokens {
    /// The main entrypoint for the proc macro which can take and can return `proc_macro::TokenStream`
    fn proc_macro_derive_base<T>(input: T) -> T
    where
        TokenStream: From<T>,
        T: From<TokenStream>,
    {
        syn::parse2::<syn::DeriveInput>(input.into())
            .and_then(Self::try_from_derive_input)
            .map(Self::into_token_stream)
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    }

    /// Attempt to construct self from a [syn::DeriveInput], assuming it meets the requirements of this proc macro.
    fn try_from_derive_input(di: syn::DeriveInput) -> syn::Result<Self>;
}
