use std::fmt::Display;

use syn::spanned::Spanned;

pub trait SpanErrorExt: Spanned + Sized {
    fn try_into_spanned_error<T>(self) -> syn::Result<T>
    where
        T: TryFrom<Self>,
        T::Error: Display,
    {
        self.try_map_spanned_error(T::try_from)
    }

    fn try_map_spanned_error<F, T, E>(self, f: F) -> syn::Result<T>
    where
        F: FnOnce(Self) -> Result<T, E>,
        E: Display,
    {
        let span = self.span();
        f(self).map_err(|e| span.spanned_error(e))
    }

    fn spanned_err<T, E>(&self, e: E) -> syn::Result<T>
    where
        E: Display,
    {
        Err(self.spanned_error(e))
    }

    fn spanned_error<E>(&self, e: E) -> syn::Error
    where
        E: Display,
    {
        syn::Error::new(self.span(), e)
    }
}

impl<T> SpanErrorExt for T where T: Spanned {}
