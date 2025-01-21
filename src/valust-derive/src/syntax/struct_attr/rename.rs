use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Ident, Lit, Meta, parse_str};

const META_SYNTAX_ERR_RENAME: &str = "\
    Invalid `rename` usage.\n\
    For usages, refer to the crate's doc.";

pub fn parse_rename(meta: &Meta, rename: &mut Option<Ident>) -> syn::Result<()> {
    match meta {
        Meta::List(lst) => {
            if let Some(rename) = rename {
                return Err(syn::Error::new(
                    rename.span(),
                    "found multiple `rename` attributes",
                ));
            }
            *rename = Some(lst.parse_args()?);
            Ok(())
        }
        Meta::NameValue(nv) => match &nv.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit), ..
            }) => {
                if let Some(rename) = rename {
                    return Err(syn::Error::new(
                        rename.span(),
                        "found multiple `rename` attributes",
                    ));
                }
                *rename = Some(parse_str(&lit.value())?);
                Ok(())
            }
            _ => Err(syn::Error::new(nv.span(), "expected string literal")),
        },
        _ => Err(syn::Error::new(meta.span(), META_SYNTAX_ERR_RENAME)),
    }
}
