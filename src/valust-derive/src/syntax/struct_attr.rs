use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Expr, ExprLit, Ident, Lit, Meta, Path, Token, parse_str};

const META_SYNTAX_ERR_RENAME: &str = "\
    Invalid `rename` usage.\n\
    For usages, refer to the crate's doc.";

pub struct StructAttr {
    pub rename: Option<Ident>,
    pub forward_derive: Vec<Path>,
}

impl StructAttr {
    pub fn from_attrs<'a>(
        attrs: impl Iterator<Item = &'a Attribute>,
    ) -> syn::Result<Self> {
        let mut rename: Option<Ident> = None;
        let mut forward_derive: Vec<Path> = vec![];

        for attr in attrs {
            if attr.path().is_ident("rename") {
                match &attr.meta {
                    Meta::List(lst) => {
                        if let Some(rename) = rename {
                            return Err(syn::Error::new(
                                rename.span(),
                                "found multiple `rename` attributes",
                            ));
                        }
                        rename = Some(lst.parse_args()?);
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
                            rename = Some(parse_str(&lit.value())?);
                        }
                        _ => {
                            return Err(syn::Error::new(
                                nv.span(),
                                "expected string literal",
                            ));
                        }
                    },
                    _ => {
                        return Err(syn::Error::new(
                            attr.span(),
                            META_SYNTAX_ERR_RENAME,
                        ));
                    }
                }
            }

            if attr.path().is_ident("forward_derive") {
                let lst = attr.meta.require_list()?;
                let args = lst
                    .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
                forward_derive.extend(args.into_iter());
            }
        }

        Ok(Self {
            rename,
            forward_derive,
        })
    }
}
