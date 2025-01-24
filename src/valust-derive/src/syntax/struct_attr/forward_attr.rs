use syn::punctuated::Punctuated;
use syn::{Meta, Token};

pub fn parse_forward_attr(
    meta: &Meta,
    forward_attr: &mut Vec<Meta>,
) -> syn::Result<()> {
    let lst = meta.require_list()?;
    let out = lst.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
    forward_attr.extend(out);

    Ok(())
}
