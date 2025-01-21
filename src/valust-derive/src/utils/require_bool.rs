use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{Expr, Lit};

const EXPR_NOT_LIT_ERROR: &str = "expect literal value";
const LIT_NOT_BOOL_OR_STR_ERROR: &str = "expect boolean literal or string";
const STR_NOT_BOOL_ERROR: &str = "expect `\"true\"` or `\"false\"`";

pub fn require_bool(tt: ParseStream) -> syn::Result<bool> {
    let expr: Expr = tt.parse()?;
    require_bool_expr(expr)
}

pub fn require_bool_expr(expr: Expr) -> syn::Result<bool> {
    let Expr::Lit(lit) = expr else {
        return Err(syn::Error::new(expr.span(), EXPR_NOT_LIT_ERROR));
    };
    match lit.lit {
        Lit::Bool(b) => Ok(b.value),
        Lit::Str(s) => match s.value().to_ascii_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(syn::Error::new(s.span(), STR_NOT_BOOL_ERROR)),
        },
        n => Err(syn::Error::new(n.span(), LIT_NOT_BOOL_OR_STR_ERROR)),
    }
}
