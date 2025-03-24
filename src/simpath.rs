#[derive(Debug)]
pub struct SimplePath(pub syn::Ident);

impl TryFrom<syn::Path> for SimplePath {
    type Error = syn::Path;

    fn try_from(sp: syn::Path) -> Result<Self, Self::Error> {
        if sp.leading_colon.is_none()
            && sp.segments.len() == 1
            && sp.segments.first().unwrap().arguments.is_none()
        {
            Ok(SimplePath(sp.segments.into_iter().next().unwrap().ident))
        } else {
            Err(sp)
        }
    }
}

impl PartialEq<&str> for SimplePath {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}
