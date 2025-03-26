use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;

use crate::ext::{PathExt as _, SpanErrorExt as _};

/// An input to a `ProcMacroDerive` impl
pub struct DeriveInput<D>
where
    D: ProcMacroDerive,
{
    /// The parsed attrs, where `A` is the [ProcMacroDerive::Attr] type
    pub attrs: Vec<D::Attr>,
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
///
/// An implementation defines an `impl` of this trait, and then the main entry point for the proc-macro lib delegates to [ProcMacroDerive::expand].
///
/// # Example
///
/// If we have implemented this trait with `MyMacroImpl` to provide `#[derive(SomeTrait)]` support, relying on the custom attribute `my_atrt`, then the entrypoint looks like this:
///
/// ```no_compile
/// #[proc_macro_derive(SomeTrait, attributes(my_attr))]
/// pub fn derive_from_via(input: TokenStream) -> TokenStream {
///     MyMacroImpl::expand(input)
/// }
/// ```
pub trait ProcMacroDerive:
    Sized + TryFrom<DeriveInput<Self>, Error = syn::Error> + ToTokens
{
    /// The custom attribute name
    const ATTR_NAME: &'static str;

    /// A custom helper attribute type for this proc macro
    type Attr: Parse;

    /// The main entrypoint for the proc macro which can take and can return `proc_macro::TokenStream`
    fn expand<T>(input: T) -> T
    where
        TokenStream: From<T>,
        T: From<TokenStream>,
    {
        syn::parse2::<DeriveInput<_>>(input.into())
            .and_then(Self::try_from)
            .map(Self::into_token_stream)
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    }
}

impl<D> Parse for DeriveInput<D>
where
    D: ProcMacroDerive,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let syn::DeriveInput { attrs, vis, ident, generics, data } =
            syn::DeriveInput::parse(input)?;

        let attrs = filter_and_try_parse_attrs::<D>(attrs)?;

        Ok(DeriveInput { attrs, vis, ident, generics, data })
    }
}

fn filter_and_try_parse_attrs<D>(attrs: Vec<syn::Attribute>) -> syn::Result<Vec<D::Attr>>
where
    D: ProcMacroDerive,
{
    attrs
        .into_iter()
        .filter_map(opt_try_from_attr::<D>)
        .collect()
}

fn opt_try_from_attr<D>(attr: syn::Attribute) -> Option<syn::Result<D::Attr>>
where
    D: ProcMacroDerive,
{
    if attr.path().eq_simple_path(D::ATTR_NAME) {
        Some(match attr.meta {
            syn::Meta::List(ml) => syn::parse2(ml.tokens),

            _ => attr.spanned_err(format!(
                "expected \"{name}(...)\" syntax",
                name = D::ATTR_NAME
            )),
        })
    } else {
        None
    }
}
