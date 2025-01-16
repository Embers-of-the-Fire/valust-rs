use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Token, Type, parenthesized, token};

use crate::config::field_config::{
    FieldManualOperation, FieldOperation, FieldOperationType,
};
use crate::utils::parser::Expression;

pub struct TransformerAttr {
    pub items: Punctuated<TransformerItem, Token![,]>,
}

impl Parse for TransformerAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: input.parse_terminated(TransformerItem::parse, Token![,])?,
        })
    }
}

impl TransformerAttr {
    pub fn into_operations(self) -> Vec<FieldOperation> {
        self.items
            .into_iter()
            .map(|t| t.into_operation().into())
            .collect()
    }
}

pub struct TransformerExpr {
    pub from_type: Type,
    pub _trans: Token![=>],
    pub expr: Expression,
}

impl Parse for TransformerExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            from_type: input.parse()?,
            _trans: input.parse()?,
            expr: input.parse()?,
        })
    }
}

pub enum TransformerItem {
    Plain(PlainTransformer),
    Fallible(FallibleTransformer),
}

impl Parse for TransformerItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![try]) {
            Ok(Self::Fallible(input.parse()?))
        } else {
            Ok(Self::Plain(input.parse()?))
        }
    }
}

impl TransformerItem {
    pub fn into_operation(self) -> FieldManualOperation {
        match self {
            Self::Plain(PlainTransformer { expr }) => FieldManualOperation {
                ty: FieldOperationType::Transform {
                    from_ty: expr.from_type,
                },
                expr: expr.expr,
                message: None,
                fallible: false,
            },
            Self::Fallible(FallibleTransformer { expr, message, .. }) => {
                FieldManualOperation {
                    ty: FieldOperationType::Transform {
                        from_ty: expr.from_type,
                    },
                    expr: expr.expr,
                    message,
                    fallible: true,
                }
            }
        }
    }
}

pub struct PlainTransformer {
    pub expr: TransformerExpr,
}

impl Parse for PlainTransformer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
        })
    }
}

pub struct FallibleTransformer {
    pub _prefix: Token![try],
    pub _paren: token::Paren,
    pub expr: TransformerExpr,
    pub message: Option<Expr>,
}

impl Parse for FallibleTransformer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _prefix: input.parse()?,
            _paren: parenthesized!(content in input),
            expr: content.parse()?,
            message: {
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                    Some(content.parse()?)
                } else {
                    None
                }
            },
        })
    }
}
