use derive_new::new;
use sappho_continuation::{Continuation, Step};
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

pub(crate) trait EvalInScope<'x, FX, C>
where
    FX: Effect,
{
    fn eval_in_scope(&'x self, scope: &Scope) -> Step<Value, &'x Expr<FX>, C>;
}

impl<'s, 'x, FX, C, X> Continuation<Value, &'x Expr<FX>, C> for Scoped<'s, X>
where
    FX: Effect,
    X: EvalInScope<'x, FX, C> + 'x,
{
    fn continue_eval(self) -> Step<Value, &'x Expr<FX>, C> {
        self.data.eval_in_scope(self.scope)
    }
}

#[derive(Debug, new)]
pub(crate) struct Scoped<'s, T> {
    pub(crate) scope: &'s Scope,
    pub(crate) data: T,
}

pub(crate) trait ScopeExt {
    fn wrap<'s, T>(&'s self, data: T) -> Scoped<'s, T>;
}

impl ScopeExt for Scope {
    fn wrap<'s, T>(&'s self, data: T) -> Scoped<'s, T> {
        Scoped::new(self, data)
    }
}
