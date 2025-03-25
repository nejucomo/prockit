use crate::{FromAttr, ProcMacroDeriveBase};

/// An input to a `ProcMacroDerive` impl
pub struct PmdInput<A>
where
    A: FromAttr,
{
    /// The parsed attrs, where `A` is the [ProcMacroDerive::Attr] type
    pub attrs: Vec<A>,
    /// The item visibility
    pub vis: syn::Visibility,
    /// The item's ident
    pub ident: syn::Ident,
    /// The item's generic parameters and bounds
    pub generics: syn::Generics,
    /// The item's data definition clause
    pub data: syn::Data,
}

/// The implementation of a `derive(MyTrait)` proc macro
pub trait ProcMacroDerive: ProcMacroDeriveBase {
    /// A custom helper attr type for this proc macro
    ///
    /// TODO: Replace `FromAttr` with `Parse` and take the name from this trait
    type Attr: FromAttr;

    /// Attempt to construct this impl from the input with pre-parsed [Self::Attr] attributes
    fn try_from_derive_with_attrs_input(dwai: PmdInput<Self::Attr>) -> syn::Result<Self>;
}

impl<T> ProcMacroDeriveBase for T
where
    T: ProcMacroDerive,
{
    fn try_from_derive_input(di: syn::DeriveInput) -> syn::Result<Self> {
        let syn::DeriveInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        } = di;

        let attrs = <T as ProcMacroDerive>::Attr::try_from_attrs(attrs)?;

        Self::try_from_derive_with_attrs_input(PmdInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        })
    }
}
