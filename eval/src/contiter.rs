use derive_new::new;
use sappho_east::Wise;
use sappho_effect::Effect;
use sappho_value::Value;

use crate::contexpr::ContExpr;
use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;
use crate::step::Step;

pub(crate) trait IterContinuation<'x, FX, I, A>: Into<Value> + Extend<(A, Value)>
where
    FX: Effect + 'x,
    I: Iterator,
    ContIter<Self, I, A>: Into<ContExpr<'x, FX>>,
{
    fn eval_via_iter(self, mut it: I) -> ExprStep<'x, FX> {
        if let Some(item) = it.next() {
            let (aux, expr) = Self::split_item(item);
            Step::continue_via(expr, ContIter::new(self, it, aux).into())
        } else {
            Step::produce(self)
        }
    }

    fn split_item(item: I::Item) -> (A, &'x Wise<FX>);
}

#[derive(Debug, new)]
pub(crate) struct ContIter<C, I, A> {
    ic: C,
    it: I,
    aux: A,
}

impl<'x, FX, C, I, A> Eval<Value, ExprStep<'x, FX>> for ContIter<C, I, A>
where
    FX: Effect + 'x,
    C: IterContinuation<'x, FX, I, A>,
    I: Iterator,
    ContIter<C, I, A>: Into<ContExpr<'x, FX>>,
{
    fn eval(self, input: Value) -> ExprStep<'x, FX> {
        let ContIter { mut ic, it, aux } = self;
        ic.extend(Some((aux, input)));
        ic.eval_via_iter(it)
    }
}
