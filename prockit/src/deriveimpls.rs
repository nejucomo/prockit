use proc_macro2::TokenStream;
use quote::ToTokens;

/// A type which implements `derive` for a given trait
pub trait DeriveImpls: TryFrom<syn::DeriveInput, Error = syn::Error> + ToTokens {
    fn derive_impls<T>(input: T) -> T
    where
        TokenStream: From<T>,
        T: From<TokenStream>,
    {
        syn::parse2::<syn::DeriveInput>(input.into())
            .and_then(Self::try_from)
            .map(Self::into_token_stream)
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    }
}
