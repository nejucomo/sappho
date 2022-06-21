use crate::ScopeRef;
use sappho_east::GenExpr;
use std::rc::Rc;

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
