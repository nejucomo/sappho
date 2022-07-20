use crate::ScopeRef;
use sappho_ast_reduced::Expr;

/// Bind a source expression to the runtime scope in which it appears for later evaluation.
///
/// # Query Example:
///
/// ```ignore
/// query (
///     let q = query 42;
///     let r = query $q;
///     $r
/// )
/// ```
///
/// When the value of `r` is evaluated a thunk captures the expression `$q` as well as the scope
/// containing `q`. When `$r` is evaluated, the thunk of the `r` query is evaluated with it's
/// definition scope.
pub struct GenThunk<Effects> {
    expr: Expr<Effects>,
    scope: ScopeRef,
}

impl<FX> GenThunk<FX> {
    pub fn new(expr: Expr<FX>, scope: ScopeRef) -> Self {
        GenThunk { scope, expr }
    }

    pub fn peek(&self) -> (&Expr<FX>, &ScopeRef) {
        (&self.expr, &self.scope)
    }
}
