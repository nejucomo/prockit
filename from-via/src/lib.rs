use proc_macro::TokenStream as TokenStream1;

use prockit::{
    DeriveInput, ProcMacroDerive,
    proc_macro2::TokenStream as TokenStream2,
    quote::{ToTokens, quote},
    syn::{
        self,
        parse::{Parse, ParseStream},
    },
};

#[proc_macro_derive(From, attributes(from_via))]
pub fn derive_from_via(input: TokenStream1) -> TokenStream1 {
    FromVia::expand(input)
}

struct FromVia {
    ident: syn::Ident,
    generics: syn::Generics,
    targets: Vec<TargetVia>,
}

struct TargetVia {
    source: syn::Type,
    transform: syn::Expr,
}

impl ProcMacroDerive for FromVia {
    const ATTR_NAME: &'static str = "from_via";

    type Attr = TargetVia;
}

impl ToTokens for FromVia {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let FromVia { ident, generics, targets } = self;

        for TargetVia { source, transform, .. } in targets.iter() {
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

impl TryFrom<DeriveInput<Self>> for FromVia {
    type Error = syn::Error;

    fn try_from(input: DeriveInput<Self>) -> Result<Self, Self::Error> {
        let DeriveInput { attrs: targets, ident, generics, .. } = input;

        Ok(FromVia { ident, generics, targets })
    }
}

impl Parse for TargetVia {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let source = syn::Type::parse(input)?;
        <syn::Token![=>]>::parse(input)?;
        let transform = syn::Expr::parse(input)?;
        Ok(TargetVia { source, transform })
    }
}
