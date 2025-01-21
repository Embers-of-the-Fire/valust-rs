use proc_macro2::TokenStream;
use syn::{Attribute, Expr, Ident, LitStr, Path, Type};

mod forward_derive;
mod post;
mod pre;
mod rename;

pub struct StructAttr {
    pub rename: Option<Ident>,
    pub forward_derive: Vec<Path>,
    pub pre: Vec<(Expr, Option<LitStr>)>,
    pub post: Vec<(Expr, Option<LitStr>)>,
}

impl StructAttr {
    pub fn from_attrs<'a>(
        attrs: impl Iterator<Item = &'a Attribute>,
    ) -> syn::Result<Self> {
        let mut rename: Option<Ident> = None;
        let mut forward_derive: Vec<Path> = vec![];
        let mut pre: Vec<(Expr, Option<LitStr>)> = vec![];
        let mut post: Vec<(Expr, Option<LitStr>)> = vec![];

        for attr in attrs {
            if attr.path().is_ident("rename") {
                rename::parse_rename(&attr.meta, &mut rename)?;
            }

            if attr.path().is_ident("forward_derive") {
                forward_derive::parse_forward_derive(&attr.meta, &mut forward_derive)?;
            }

            if attr.path().is_ident("pre") {
                pre::parse_pre(&attr.meta, &mut pre)?;
            }

            if attr.path().is_ident("post") {
                post::parse_post(&attr.meta, &mut post)?;
            }
        }

        Ok(Self {
            rename,
            forward_derive,
            pre,
            post,
        })
    }

    pub fn gen_pre_expr(
        &self,
        name: &Ident,
        fields: impl Iterator<Item = (Ident, Type)>,
    ) -> (Ident, TokenStream) {
        pre::gen_pre_expr(self.pre.iter(), name, fields)
    }

    pub fn gen_post_expr<'a>(
        &self,
        name: &Ident,
        fields: impl Iterator<Item = (Ident, &'a Type)>,
    ) -> (Ident, TokenStream) {
        post::gen_post_expr(self.post.iter(), name, fields)
    }
}
