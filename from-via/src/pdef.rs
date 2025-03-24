use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::Parse;

use crate::target::{TargetVia, TargetsVia};
use crate::typedef::TypeDef;

pub struct ParsedDef {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub targets: TargetsVia,
}

impl ToTokens for ParsedDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ParsedDef {
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

impl Parse for ParsedDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let td = TypeDef::parse(input)?;
        Self::try_from(td)
    }
}

impl TryFrom<TypeDef> for ParsedDef {
    type Error = syn::Error;

    fn try_from(td: TypeDef) -> syn::Result<Self> {
        use TypeDef::*;

        match td {
            Type(x) => Self::try_from(x),
            Struct(x) => Self::try_from(x),
            Enum(x) => Self::try_from(x),
        }
    }
}

impl TryFrom<syn::ItemType> for ParsedDef {
    type Error = syn::Error;

    fn try_from(item: syn::ItemType) -> syn::Result<Self> {
        let syn::ItemType {
            attrs,
            ident,
            generics,
            ..
        } = item;
        Self::try_from((ident, generics, attrs))
    }
}

impl TryFrom<syn::ItemStruct> for ParsedDef {
    type Error = syn::Error;

    fn try_from(item: syn::ItemStruct) -> syn::Result<Self> {
        let syn::ItemStruct {
            attrs,
            ident,
            generics,
            ..
        } = item;
        Self::try_from((ident, generics, attrs))
    }
}

impl TryFrom<syn::ItemEnum> for ParsedDef {
    type Error = syn::Error;

    fn try_from(item: syn::ItemEnum) -> syn::Result<Self> {
        let syn::ItemEnum {
            attrs,
            ident,
            generics,
            ..
        } = item;
        Self::try_from((ident, generics, attrs))
    }
}

impl TryFrom<(syn::Ident, syn::Generics, Vec<syn::Attribute>)> for ParsedDef {
    type Error = syn::Error;

    fn try_from(
        (ident, generics, attrs): (syn::Ident, syn::Generics, Vec<syn::Attribute>),
    ) -> syn::Result<Self> {
        let targets = TargetsVia::try_from(attrs)?;
        Ok(Self {
            ident,
            generics,
            targets,
        })
    }
}
