use prockit::FromAttr;
use syn::parse::{Parse, ParseStream};

pub struct TargetVia {
    pub source: syn::Type,
    pub transform: syn::Expr,
}

impl FromAttr for TargetVia {
    fn attr_name() -> &'static str {
        "from_via"
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
