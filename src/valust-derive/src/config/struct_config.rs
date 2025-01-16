use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::spanned::Spanned;
use syn::{Attribute, Expr, ExprLit, Lit, Meta, Visibility, parse_str, parse2};

use crate::config::field_config::FieldConfig;
use crate::parse::parse_field_validator::ValidatorItem;
use crate::parse::parse_struct::{
    StructForwardDerive, StructPostValidatorAttr, StructPreValidatorAttr,
};
use crate::utils::create_error::create_meta_validate_error;
use crate::utils::iter::IteratorExt;
use crate::utils::option::{OptionErrExt, OptionExt};
use crate::utils::parse_error::{
    AttrPlacement, create_invalid_attr_call_error, create_misplaced_error,
    get_attr_placement,
};
use crate::utils::parser::Expression;

pub struct StructConfig {
    pub name: Ident,
    pub vis: Visibility,
    pub attrs: StructAttr,
    pub fields: Vec<FieldConfig>,
    pub is_named: bool,
}

impl StructConfig {
    fn to_origin_decl(&self) -> (Ident, TokenStream) {
        let name = self.attrs.rename.clone().unwrap_or_else(|| {
            format_ident!("Raw{}", self.name, span = self.name.span())
        });
        let vis = &self.vis;
        let fields = self.fields.iter().map(|t| t.to_origin_decl(self.is_named));
        let derives = &self.attrs.forward_derive;
        let derive_code = if !derives.is_empty() {
            quote! {
                #[derive(#(#derives),*)]
            }
        } else {
            quote! {}
        };

        (
            name.clone(),
            if self.is_named {
                quote! {
                    #derive_code
                    #vis struct #name { #(#fields),* }
                }
            } else {
                quote! {
                    #derive_code
                    #vis struct #name(#(#fields),*);
                }
            },
        )
    }

    pub fn to_trait_impl(&self) -> syn::Result<TokenStream> {
        let (origin_name, origin_decl) = self.to_origin_decl();
        let raw_ident =
            format_ident!("_valust_raw_{}", origin_name, span = origin_name.span());
        let name = &self.name;

        let unpack = self.to_unpack_op(&raw_ident, &origin_name);
        let process = self.to_process_op()?;
        let pack = self.to_pack_op()?;

        Ok(quote! {
            #[automatically_derived]
            #origin_decl

            #[automatically_derived]
            impl ::valust::Validate for #name {
                type Raw = #origin_name;

                fn validate(#raw_ident: Self::Raw)
                    -> ::std::result::Result<#name, ::valust::error::ValidationError> {
                    #![allow(non_snake_case)]
                    #unpack
                    #process
                    #pack
                }
            }
        })
    }

    fn to_unpack_op(&self, raw_ident: &Ident, origin_ident: &Ident) -> TokenStream {
        let assign = self.fields.iter().map(|f| f.name.to_ident());
        if self.is_named {
            quote! { let #origin_ident { #(#assign),* } = #raw_ident; }
        } else {
            quote! { let #origin_ident(#(#assign),*) = #raw_ident; }
        }
    }

    fn to_process_op(&self) -> syn::Result<TokenStream> {
        let error_ident =
            format_ident!("__valust_error_{}", self.name, span = self.name.span());

        let pre_check = self
            .attrs
            .pre
            .iter()
            .map(|op| op.to_process_op(&self.name, &error_ident));
        let post_check = self
            .attrs
            .post
            .iter()
            .map(|op| op.to_process_op(&self.name, &error_ident));
        let unwrap_ops = self.to_unwrap_op()?;

        let (field_ops, error) = self
            .fields
            .iter()
            .map(|f| (f, f.to_process_ops()))
            .map(|(field, t)| {
                t.map(|(func, tt)| {
                    let ident = field.name.to_ident();
                    let ty = &field.ty;
                    quote! {
                        #tt
                        let #ident: ::std::option::Option<#ty> = #func(#ident, &mut #error_ident);
                    }
                })
            })
            .collect_result();

        error.err_or(())?;

        Ok(quote! {
            let mut #error_ident = ::valust::error::ValidationError::new();

            #(#pre_check)*
            #(#field_ops)*

            #error_ident.check()?;
            let mut #error_ident = ::valust::error::ValidationError::new();

            #unwrap_ops
            #(#post_check)*

            #error_ident.check()?;
        })
    }

    fn to_unwrap_op(&self) -> syn::Result<TokenStream> {
        let (key, exp): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .map(|f| {
                (
                    f.name.to_ident(),
                    format!(
                        "Unexpected error occurred while processing field `{}`",
                        f.name.to_ident_assign()
                    ),
                )
            })
            .unzip();
        Ok(quote! {
            #(let #key = #key.expect(#exp);)*
        })
    }

    fn to_pack_op(&self) -> syn::Result<TokenStream> {
        let (key, assign): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .map(|f| (f.name.to_ident(), f.name.to_ident_assign()))
            .unzip();
        let name = &self.name;
        Ok(quote! {
            ::std::result::Result::Ok(#name { #(#assign: #key),* })
        })
    }
}

pub struct StructOperation {
    pub expr: Expr,
    pub message: Option<Expr>,
    pub fallible: bool,
}

impl StructOperation {
    pub fn from_validator_item(item: ValidatorItem) -> syn::Result<Self> {
        fn func_only(expr: Expression) -> syn::Result<Expr> {
            match expr {
                Expression::Expr(e) => Ok(e),
                Expression::Func(f) => {
                    Err(syn::Error::new(f.span(), "function is not allowed here"))
                }
                #[cfg(feature = "regex")]
                Expression::Regex(r) => Err(syn::Error::new(
                    r.span(),
                    "regex expression is not allowed here",
                )),
            }
        }

        match item {
            ValidatorItem::Plain(plain) => Ok(Self {
                expr: func_only(plain.expr.expr)?,
                message: None,
                fallible: false,
            }),
            ValidatorItem::Message(message) => Ok(Self {
                expr: func_only(message.expr.expr)?,
                message: Some(message.message),
                fallible: false,
            }),
            ValidatorItem::Fallible(fallible) => Ok(Self {
                expr: func_only(fallible.expr.expr)?,
                message: fallible.message,
                fallible: true,
            }),
            #[cfg(feature = "regex")]
            ValidatorItem::Regex(regex) => Ok(Self {
                expr: func_only(regex.text)?,
                message: regex.message,
                fallible: false,
            }),
        }
    }

    pub fn to_process_op(
        &self,
        struct_name: &Ident,
        error_ident: &Ident,
    ) -> TokenStream {
        let expr = &self.expr;

        let err =
            format_ident!("__valust_err_{}", struct_name, span = struct_name.span());

        let code_invalid =
            create_meta_validate_error(error_ident, None, self.message.as_ref());
        let code_error =
            create_meta_validate_error(error_ident, Some(&err), self.message.as_ref());

        if self.fallible {
            quote! {
                match (#expr) {
                    ::std::result::Result::Ok(true) => {},
                    ::std::result::Result::Ok(false) => {
                        #code_invalid;
                    },
                    ::std::result::Result::Err(#err) => {
                        #code_error;
                    },
                }
            }
        } else {
            quote! {
                if !(#expr) {
                    #code_invalid;
                }
            }
        }
    }
}

#[derive(Default)]
pub struct StructAttr {
    pub rename: Option<Ident>,
    pub forward_derive: Vec<Ident>,
    pub pre: Vec<StructOperation>,
    pub post: Vec<StructOperation>,
}

impl StructAttr {
    pub fn from_attrs<'a>(
        attrs: impl Iterator<Item = &'a Attribute>,
    ) -> syn::Result<Self> {
        let mut st_attr = StructAttr::default();
        let mut error: Option<syn::Error> = None;
        for attr in attrs {
            let span = attr.path().span();
            if attr.path().is_ident("pre") {
                if let Meta::List(list) = &attr.meta {
                    match parse2::<StructPreValidatorAttr>(list.tokens.clone()) {
                        Ok(t) => {
                            let (mut pre, err) = t
                                .pre
                                .into_iter()
                                .map(StructOperation::from_validator_item)
                                .collect_result();
                            err.err_or(())?;
                            st_attr.pre.append(&mut pre);
                        }
                        Err(e) => {
                            error.insert_or_combine(e);
                        }
                    }
                } else {
                    error.insert_or_combine(create_invalid_attr_call_error(
                        span,
                        "pre",
                        &["pre(...)"],
                    ));
                }
            } else if attr.path().is_ident("post") {
                if let Meta::List(list) = &attr.meta {
                    match parse2::<StructPostValidatorAttr>(list.tokens.clone()) {
                        Ok(t) => {
                            let (mut post, err) = t
                                .post
                                .into_iter()
                                .map(StructOperation::from_validator_item)
                                .collect_result();
                            err.err_or(())?;
                            st_attr.post.append(&mut post);
                        }
                        Err(e) => {
                            error.insert_or_combine(e);
                        }
                    }
                } else {
                    error.insert_or_combine(create_invalid_attr_call_error(
                        span,
                        "post",
                        &["post(...)"],
                    ));
                }
            } else if attr.path().is_ident("forward_derive") {
                if let Meta::List(list) = &attr.meta {
                    match parse2::<StructForwardDerive>(list.tokens.clone()) {
                        Ok(t) => st_attr.forward_derive.extend(t.derives),
                        Err(e) => {
                            error.insert_or_combine(e);
                        }
                    }
                } else {
                    error.insert_or_combine(create_invalid_attr_call_error(
                        span,
                        "forward_derive",
                        &["forward_derive(...)"],
                    ));
                }
            } else if attr.path().is_ident("rename") {
                let ty = match &attr.meta {
                    Meta::List(list) => match parse2::<Ident>(list.tokens.clone()) {
                        Ok(t) => Some(t),
                        Err(e) => {
                            error.insert_or_combine(e);
                            None
                        }
                    },
                    Meta::NameValue(nv) => match &nv.value {
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(lit), ..
                        }) => match parse_str(&lit.value()) {
                            Ok(t) => Some(t),
                            Err(e) => {
                                error.insert_or_combine(e);
                                None
                            }
                        },
                        _ => {
                            error.insert_or_combine(syn::Error::new(
                                nv.value.span(),
                                "`rename` value must be wrapped in a string if you use the syntax `rename = ...`",
                            ));
                            None
                        }
                    },
                    _ => {
                        error.insert_or_combine(create_invalid_attr_call_error(
                            span,
                            "rename",
                            &["rename = \"...\"", "rename(...)"],
                        ));
                        None
                    }
                };
                match (&mut st_attr.rename, ty) {
                    (Some(_), Some(_)) => error.insert_or_combine(syn::Error::new(
                        span,
                        "found multiple `rename` attribute.",
                    )),
                    (n @ None, Some(r)) => *n = Some(r),
                    _ => {}
                }
            } else if matches!(
                get_attr_placement(attr.path()),
                Some(AttrPlacement::Field)
            ) {
                error.insert_or_combine(create_misplaced_error(
                    span,
                    &attr.path().to_token_stream().to_string(),
                    AttrPlacement::Field,
                ));
            }
        }

        error.err_or(())?;
        Ok(st_attr)
    }
}
