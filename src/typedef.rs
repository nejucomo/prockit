use syn::parse::Parse;
use syn::spanned::Spanned as _;

pub enum TypeDef {
    Type(syn::ItemType),
    Struct(syn::ItemStruct),
    Enum(syn::ItemEnum),
}

impl TryFrom<syn::Item> for TypeDef {
    type Error = syn::Error;

    fn try_from(it: syn::Item) -> Result<Self, Self::Error> {
        use syn::Item::{Enum, Struct, Type};

        match it {
            Enum(x) => Ok(Self::Enum(x)),
            Struct(x) => Ok(Self::Struct(x)),
            Type(x) => Ok(Self::Type(x)),
            _ => Err(syn::Error::new(
                it.span(),
                "only enum, struct, and type items supported",
            )),
        }
    }
}

impl Parse for TypeDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let it = syn::Item::parse(input)?;
        Self::try_from(it)
    }
}
