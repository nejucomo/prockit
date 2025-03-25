#![allow(missing_docs)]

use syn::parse::Parse;

use crate::ext::{PathExt as _, SpanErrorExt as _};

/// An attribute type for [ProcMacroDerive](crate::ProcMacroDerive) implementations
pub trait FromAttr: Parse {
    /// The name of this attr
    fn attr_name() -> &'static str;

    fn try_from_attrs(attrs: Vec<syn::Attribute>) -> syn::Result<Vec<Self>> {
        attrs.into_iter().filter_map(Self::opt_try_from).collect()
    }

    fn opt_try_from(attr: syn::Attribute) -> Option<syn::Result<Self>> {
        if attr.path().eq_simple_path(Self::attr_name()) {
            Some(match attr.meta {
                syn::Meta::List(ml) => syn::parse2(ml.tokens),
                _ => attr.spanned_err(format!("expected \"{}(...)\" syntax", Self::attr_name())),
            })
        } else {
            None
        }
    }
}
