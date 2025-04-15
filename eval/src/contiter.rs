use derive_new::new;
use sappho_east::Wise;
use sappho_effect::Effect;
use sappho_value::Value;

use crate::contexpr::ContExpr;
use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;
use crate::step::Step;

pub(crate) fn continue_eval_via_iter<'x, FX, V, I, A>(value: V, mut it: I) -> ExprStep<'x, FX>
where
    FX: Effect,
    V: Into<Value>,
    I: Iterator<Item = (A, &'x Wise<FX>)>,
    ContIter<V, I, A>: Into<ContExpr<'x, FX>>,
{
    if let Some((aux, expr)) = it.next() {
        Step::continue_via(expr, ContIter::new(value, it, aux).into())
    } else {
        Step::produce(value)
    }
}

#[derive(Debug, new)]
pub(crate) struct ContIter<V, I, A> {
    value: V,
    it: I,
    aux: A,
}

impl<'x, FX, V, I, A> Eval<Value, ExprStep<'x, FX>> for ContIter<V, I, A>
where
    FX: Effect,
    V: Into<Value> + Extend<(A, Value)>,
    I: Iterator<Item = (A, &'x Wise<FX>)>,
    ContIter<V, I, A>: Into<ContExpr<'x, FX>>,
{
    fn eval(self, input: Value) -> ExprStep<'x, FX> {
        let ContIter { mut value, it, aux } = self;
        value.extend(Some((aux, input)));
        continue_eval_via_iter(value, it)
    }
}
