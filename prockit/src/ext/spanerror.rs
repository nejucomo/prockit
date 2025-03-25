use std::fmt::Display;

use syn::spanned::Spanned;

/// An extension to all [Spanned] types to simplify generating errors with the correct spans
pub trait SpanErrorExt: Spanned + Sized {
    /// Attempt to convert the spanned entity to `T`, using `self.span()` for any conversion failure
    ///
    /// This simplifies a very common parsing need.
    ///
    /// See [Spanned::span].
    fn try_into_spanned_error<T>(self) -> syn::Result<T>
    where
        T: TryFrom<Self>,
        T::Error: Display,
    {
        self.try_map_spanned_error(T::try_from)
    }

    /// Take self then pass it to a closure which may generate an error, which will have `self.span()`
    ///
    /// This simplifies parsing needs where [Self::try_into_spanned_error] does not suffice.
    ///
    /// See [Spanned::span].
    fn try_map_spanned_error<F, T, E>(self, f: F) -> syn::Result<T>
    where
        F: FnOnce(Self) -> Result<T, E>,
        E: Display,
    {
        let span = self.span();
        f(self).map_err(|e| span.spanned_error(e))
    }

    /// Construct an [Err] with `self.span()`
    fn spanned_err<T, E>(&self, e: E) -> syn::Result<T>
    where
        E: Display,
    {
        Err(self.spanned_error(e))
    }

    /// Construct a [syn::Error] with `self.span()`
    fn spanned_error<E>(&self, e: E) -> syn::Error
    where
        E: Display,
    {
        syn::Error::new(self.span(), e)
    }
}

impl<T> SpanErrorExt for T where T: Spanned {}
