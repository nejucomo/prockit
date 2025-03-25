use proc_macro2::TokenStream;
use prockit::{PmdInput, ProcMacroDerive};
use quote::{ToTokens, quote};

use crate::target::TargetVia;

pub struct FromVia {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub targets: Vec<TargetVia>,
}

impl ProcMacroDerive for FromVia {
    type Attr = TargetVia;

    fn try_from_derive_with_attrs_input(dwai: PmdInput<TargetVia>) -> syn::Result<Self> {
        let PmdInput {
            attrs: targets,
            ident,
            generics,
            ..
        } = dwai;

        Ok(FromVia {
            ident,
            generics,
            targets,
        })
    }
}

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
