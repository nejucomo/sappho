use derive_new::new;
use sappho_ast_effect::Effect;
use sappho_ast_reduced::Expr;

use crate::ScopeRef;

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
#[derive(Debug, new)]
pub struct GenThunk<FX>
where
    FX: Effect,
{
    expr: Expr<FX>,
    scope: ScopeRef,
}

impl<FX> GenThunk<FX>
where
    FX: Effect,
{
    pub fn peek(&self) -> (&Expr<FX>, &ScopeRef) {
        (&self.expr, &self.scope)
    }
}
