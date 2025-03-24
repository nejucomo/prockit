use syn::parse::{Parse, ParseStream};

use crate::metapivot::{AttrPivot, MetaPivotBody};

pub struct TargetsVia(Vec<TargetVia>);

pub struct TargetVia {
    pub source: syn::Type,
    #[allow(dead_code)]
    pub arrow: syn::Token![=>],
    pub transform: syn::Expr,
}

impl TargetsVia {
    pub fn iter(&self) -> impl Iterator<Item = &TargetVia> {
        self.0.iter()
    }
}

impl TryFrom<Vec<syn::Attribute>> for TargetsVia {
    type Error = syn::Error;

    fn try_from(attrs: Vec<syn::Attribute>) -> Result<Self, Self::Error> {
        Ok(Self(
            attrs
                .into_iter()
                .filter_map(|attr| {
                    AttrPivot::from(attr)
                        .filter_try_from_meta_pivot_body("from_via")
                        .transpose()
                })
                .collect::<syn::Result<Vec<_>>>()?,
        ))
    }
}

impl TryFrom<Option<MetaPivotBody>> for TargetVia {
    type Error = String;

    fn try_from(optmpb: Option<MetaPivotBody>) -> Result<Self, Self::Error> {
        if let Some(MetaPivotBody::List(ml)) = optmpb {
            syn::parse2(ml.tokens).map_err(|e| e.to_string())
        } else {
            Err("expected `#[from_via( <type> => <maker-expr> )]`".to_string())
        }
    }
}

impl Parse for TargetVia {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TargetVia {
            source: input.parse()?,
            arrow: input.parse()?,
            transform: input.parse()?,
        })
    }
}
