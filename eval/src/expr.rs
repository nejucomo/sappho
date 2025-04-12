use derive_more::From;
use sappho_continuation::Continuation;
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::eval::EvalResult;
use crate::exprstep::ExprStep;
use crate::wrapper::Ev;

impl<'s, 'x, FX> Continuation<&'s Scope, EvalResult<Value>, Ev<&'x Expr<FX>>, ContExpr<'x, FX>>
    for Ev<&'x Expr<FX>>
where
    FX: Effect + 'x,
{
    fn continue_with(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        use Expr::*;

        match self.0 {
            Prim(x) => Ev(x).continue_with(scope),
            Ref(x) => Ev(x).continue_with(scope),
            ObjectDef(x) => Ev(x).continue_with(scope),
            ListDef(x) => Ev(x).continue_with(scope),
            Let(x) => Ev(x).continue_with(scope),
            Match(x) => Ev(x).continue_with(scope),
            Application(x) => Ev(x).continue_with(scope),
            Lookup(x) => Ev(x).continue_with(scope),
            Interaction(x) => Ev(x).continue_with(scope),
        }
    }
}

#[derive(Debug, From)]
pub(crate) enum ContExpr<'x, FX> {
    Fixme(&'x FX),
}
