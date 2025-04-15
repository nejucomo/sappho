use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_primval::PrimVal;
use sappho_value::Scope;

use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;
use crate::step::Step;

impl<'s, 'x, FX> Eval<&'s Scope, ExprStep<'x, FX>> for &'x Expr<FX>
where
    FX: Effect,
{
    fn eval(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        use Expr::*;

        match self {
            Prim(x) => x.eval(scope),
            Ref(x) => x.eval(scope),
            ObjectDef(x) => x.eval(scope),
            ListDef(x) => x.eval(scope),
            other => todo!("{other:#?}"),
            // Let(x) => x.eval(scope),
            // Match(x) => x.eval(scope),
            // Application(x) => x.eval(scope),
            // Lookup(x) => x.eval(scope),
            // Interaction(x) => x.eval(scope),
        }
    }
}

impl<'x, FX, S> Eval<S, ExprStep<'x, FX>> for &'x PrimVal
where
    FX: Effect,
{
    fn eval(self, _: S) -> ExprStep<'x, FX> {
        Step::produce(*self)
    }
}

impl<'s, 'x, FX> Eval<&'s Scope, ExprStep<'x, FX>> for &'x RcId
where
    FX: Effect,
{
    fn eval(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        Step::produce(
            scope
                .lookup(self)
                .expect("TODO: eval user-space error propagation")
                .clone(),
        )
    }
}
