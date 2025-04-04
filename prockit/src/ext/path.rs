use easy_ext::ext;

/// Extend [syn::Path] with helper methods
#[ext(PathExt)]
pub impl syn::Path {
    /// Return true if this path is "simple" and equal to `s`
    ///
    /// See [Self::ref_simple_path] for the definition of "simple".
    fn eq_simple_path(&self, s: &str) -> bool {
        self.ref_simple_path().map(|id| id == s).unwrap_or(false)
    }

    /// If the path is just a simple single ident, reference that
    fn ref_simple_path(&self) -> Option<&syn::Ident> {
        if self.leading_colon.is_none() {
            let mut it = self.segments.iter();
            if let (Some(seg), None) = (it.next(), it.next()) {
                if seg.arguments.is_none() {
                    return Some(&seg.ident);
                }
            }
        }
        // It's not a "simple" path:
        None
    }
}
