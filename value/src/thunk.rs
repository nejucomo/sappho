use crate::ScopeRef;
use sappho_east::GenExpr;
use std::rc::Rc;

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
pub struct GenThunk<'a, Effects> {
    expr: &'a Rc<GenExpr<Effects>>,
    scope: ScopeRef,
}

impl<'a, FX> GenThunk<'a, FX> {
    pub fn new(expr: &'a Rc<GenExpr<FX>>, scope: ScopeRef) -> Self {
        GenThunk { scope, expr }
    }

    pub fn peek(&self) -> (&'a Rc<GenExpr<FX>>, &ScopeRef) {
        (self.expr, &self.scope)
    }
}
