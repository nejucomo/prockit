use proc_macro2::TokenStream;
use prockit::DeriveImpls;
use quote::{ToTokens, quote};

use crate::target::{TargetVia, TargetsVia};

pub struct FromVia {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub targets: TargetsVia,
}

impl DeriveImpls for FromVia {}

impl ToTokens for FromVia {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FromVia {
            ident,
            generics,
            targets,
        } = self;

        for TargetVia {
            source, transform, ..
        } in targets.iter()
        {
            quote! {
                #[automatically_derived]
                impl #generics ::core::convert::From< #source > for #ident #generics {
                    fn from(v: #source) -> Self {
                        Self::from( (#transform)(v) )
                    }
                }
            }
            .to_tokens(tokens)
        }
    }
}

impl TryFrom<syn::DeriveInput> for FromVia {
    type Error = syn::Error;

    fn try_from(di: syn::DeriveInput) -> syn::Result<Self> {
        let syn::DeriveInput {
            ident,
            generics,
            attrs,
            ..
        } = di;

        let targets = TargetsVia::try_from(attrs)?;

        Ok(Self {
            ident,
            generics,
            targets,
        })
    }
}
