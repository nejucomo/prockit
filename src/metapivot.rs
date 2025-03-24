use derive_more::{Constructor, From};
use syn::spanned::Spanned;

use crate::simpath::SimplePath;

pub struct AttrPivot {
    pub span: proc_macro2::Span,
    #[allow(dead_code)]
    pub pound_token: syn::token::Pound,
    #[allow(dead_code)]
    pub style: syn::AttrStyle,
    #[allow(dead_code)]
    pub bracket_token: syn::token::Bracket,
    pub meta_pivot: MetaPivot,
}

pub struct MetaPivot {
    pub path: syn::Path,
    pub body: Option<MetaPivotBody>,
}

#[derive(From)]
pub enum MetaPivotBody {
    List(MetaPivotList),
    NameValue(MetaPivotNameValue),
}

#[derive(Constructor)]
pub struct MetaPivotList {
    pub delimiter: syn::MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}

#[derive(Constructor)]
pub struct MetaPivotNameValue {
    pub eq_token: syn::token::Eq,
    pub value: syn::Expr,
}

impl AttrPivot {
    pub fn filter_try_from_meta_pivot_body<T>(self, name: &str) -> syn::Result<Option<T>>
    where
        T: TryFrom<Option<MetaPivotBody>>,
        T::Error: std::fmt::Display,
    {
        let AttrPivot {
            span, meta_pivot, ..
        } = self;

        if let Some((id, optmpb)) = meta_pivot.into() {
            if id == name {
                T::try_from(optmpb)
                    .map(Some)
                    .map_err(|e| syn::Error::new(span, e))
            } else {
                // Simple paths != name are ignored:
                Ok(None)
            }
        } else {
            // Complicated paths are ignored:
            Ok(None)
        }
    }
}

impl From<syn::Attribute> for AttrPivot {
    fn from(attr: syn::Attribute) -> Self {
        let span = attr.span();

        let syn::Attribute {
            pound_token,
            style,
            bracket_token,
            meta,
        } = attr;

        let meta_pivot = MetaPivot::from(meta);

        Self {
            span,
            pound_token,
            style,
            bracket_token,
            meta_pivot,
        }
    }
}

impl From<syn::Meta> for MetaPivot {
    fn from(meta: syn::Meta) -> Self {
        match meta {
            syn::Meta::Path(path) => Self::from(path),
            syn::Meta::List(ml) => Self::from(ml),
            syn::Meta::NameValue(mnv) => Self::from(mnv),
        }
    }
}

impl From<syn::Path> for MetaPivot {
    fn from(path: syn::Path) -> Self {
        Self { path, body: None }
    }
}

impl From<syn::MetaList> for MetaPivot {
    fn from(ml: syn::MetaList) -> Self {
        let syn::MetaList {
            path,
            delimiter,
            tokens,
        } = ml;

        Self {
            path,
            body: Some(MetaPivotList::new(delimiter, tokens).into()),
        }
    }
}

impl From<syn::MetaNameValue> for MetaPivot {
    fn from(mnv: syn::MetaNameValue) -> Self {
        let syn::MetaNameValue {
            path,
            eq_token,
            value,
        } = mnv;

        Self {
            path,
            body: Some(MetaPivotNameValue::new(eq_token, value).into()),
        }
    }
}

// Simplified attribute pivot:
impl From<MetaPivot> for Option<(syn::Ident, Option<MetaPivotBody>)> {
    fn from(mp: MetaPivot) -> Self {
        let SimplePath(id) = SimplePath::try_from(mp.path).ok()?;
        Some((id, mp.body))
    }
}
