pub trait OptionExt {
    type Item;

    fn err_or<T>(self, ok: T) -> Result<T, Self::Item>;
}

impl<T> OptionExt for Option<T> {
    type Item = T;

    fn err_or<U>(self, ok: U) -> Result<U, T> {
        match self {
            Some(v) => Err(v),
            None => Ok(ok),
        }
    }
}

pub trait OptionErrExt {
    fn insert_or_combine(&mut self, rhs: syn::Error);
}

impl OptionErrExt for Option<syn::Error> {
    fn insert_or_combine(&mut self, rhs: syn::Error) {
        match self {
            Some(e) => e.combine(rhs),
            None => *self = Some(rhs),
        }
    }
}
