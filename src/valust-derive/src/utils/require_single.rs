use proc_macro2::Span;

pub fn require_single_fallible<T>(
    new: Option<syn::Result<T>>,
    orig: &mut Option<T>,
    field: &str,
    span: Span,
) -> syn::Result<()> {
    if orig.is_some() && new.is_some() {
        Err(syn::Error::new(
            span,
            format!("found multiple conflicting field `{}`", field),
        ))
    } else if let Some(new) = new {
        *orig = Some(new?);
        Ok(())
    } else {
        Ok(())
    }
}
