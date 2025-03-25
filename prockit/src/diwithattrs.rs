use crate::{DeriveImpls, FromAttr};

pub struct DeriveWithAttrsInput<A>
where
    A: FromAttr,
{
    pub attrs: Vec<A>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: syn::Data,
}

pub trait DeriveImplsWithAttrs: DeriveImpls {
    type Attr: FromAttr;

    fn try_from_derive_with_attrs_input(
        dwai: DeriveWithAttrsInput<Self::Attr>,
    ) -> syn::Result<Self>;
}

impl<T> DeriveImpls for T
where
    T: DeriveImplsWithAttrs,
{
    fn try_from_derive_input(di: syn::DeriveInput) -> syn::Result<Self> {
        let syn::DeriveInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        } = di;

        let attrs = <T as DeriveImplsWithAttrs>::Attr::try_from_attrs(attrs)?;

        Self::try_from_derive_with_attrs_input(DeriveWithAttrsInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        })
    }
}
