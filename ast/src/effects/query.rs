use crate::Expr;
use sappho_unparse::{Stream, Unparse};

/// Query expressions can read mutable memory, as in `$myvar`.
pub type QueryExpr = Expr<QueryEffects>;

/// The query effect reads mutable memory.
#[derive(Debug, PartialEq)]
pub enum QueryEffects {
    /// Inquire is the name of the `$myvar` effect syntax & semantics.
    Inquire(Box<Expr<QueryEffects>>),
}

impl From<QueryEffects> for QueryExpr {
    fn from(x: QueryEffects) -> Self {
        Expr::Effect(x)
    }
}

impl Unparse for QueryEffects {
    fn unparse_into(&self, s: &mut Stream) {
        use QueryEffects::*;

        match self {
            Inquire(x) => {
                s.write("$");
                s.write(x);
            }
        }
    }
}
