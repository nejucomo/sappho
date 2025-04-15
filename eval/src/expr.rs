use sappho_continuation::{Continuation, Step};
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_primval::PrimVal;
use sappho_value::Value;

use crate::cont::Cont;
use crate::scoped::{EvalInScope, ScopeExt as _, Scoped};

impl<'s, 'x, FX> EvalInScope<'x, FX, Cont<'x, FX>> for Expr<FX>
where
    FX: Effect,
{
    fn eval_in_scope(
        &'x self,
        scope: &sappho_value::Scope,
    ) -> Step<Value, &'x Expr<FX>, Cont<'x, FX>> {
        use Expr::*;

        match self {
            Prim(x) => x.eval_in_scope(scope),
            Ref(x) => x.eval_in_scope(scope),
            ObjectDef(x) => x.eval_in_scope(scope),
            ListDef(x) => x.eval_in_scope(scope),
            Let(x) => x.eval_in_scope(scope),
            Match(x) => x.eval_in_scope(scope),
            Application(x) => x.eval_in_scope(scope),
            Lookup(x) => x.eval_in_scope(scope),
            Interaction(x) => x.eval_in_scope(scope),
        }
    }
}

impl<'s, 'x, FX> EvalInScope<'x, FX, Cont<'x, FX>> for PrimVal
where
    FX: Effect,
{
    fn eval_in_scope(&'x self, _: &sappho_value::Scope) -> Step<Value, &'x Expr<FX>, Cont<'x, FX>> {
        Step::produce(*self)
    }
}
