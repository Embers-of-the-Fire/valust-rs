use syn::punctuated::Punctuated;
use syn::{Meta, Path, Token};

pub fn parse_forward_derive(
    meta: &Meta,
    forward_derive: &mut Vec<Path>,
) -> syn::Result<()> {
    let lst = meta.require_list()?;
    let args = lst.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
    forward_derive.extend(args);

    Ok(())
}
