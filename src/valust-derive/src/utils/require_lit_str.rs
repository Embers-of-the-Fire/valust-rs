use syn::spanned::Spanned;
use syn::{Expr, Lit, LitStr};

const EXPR_NOT_LIT_ERROR: &str = "expect literal value";
const LIT_NOT_STR_ERROR: &str = "expect string literal";

pub fn require_lit_str(expr: Expr) -> syn::Result<LitStr> {
    let Expr::Lit(lit) = expr else {
        return Err(syn::Error::new(expr.span(), EXPR_NOT_LIT_ERROR));
    };
    match lit.lit {
        Lit::Str(s) => Ok(s),
        n => Err(syn::Error::new(n.span(), LIT_NOT_STR_ERROR)),
    }
}
