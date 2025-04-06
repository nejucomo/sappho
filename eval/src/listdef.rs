use derive_new::new;
use sappho_east::{ListDef, Wise};
use sappho_effect::Effect;
use sappho_list::List;
use sappho_value::{Valuable as _, Value};

use crate::continuation::{Continuation, EvalStep};
use crate::scoped::Scoped;
use crate::step::Step::{self, Continue, Produce};

#[derive(Debug, new)]
pub(crate) struct ListDefCont<FX>
where
    FX: Effect,
{
    elemexprs: <ListDef<FX> as IntoIterator>::IntoIter,
    #[new(default)]
    vals: Vec<Value>,
    is_tail: bool,
}

impl<FX> ListDefCont<FX>
where
    FX: Effect,
{
    fn build_list(self, tail: Option<Value>) -> List<Value> {
        let mut list = tail
            // BUG: Propagate exceptions into user-space?
            .map(|v| v.as_list().unwrap().clone())
            .unwrap_or_default();

        for v in self.vals.into_iter().rev() {
            list = list.prepend(v);
        }
        list
    }
}

impl<FX> EvalStep<FX> for Scoped<ListDef<FX>>
where
    FX: Effect,
{
    type Continuation = Scoped<ListDefCont<FX>>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation> {
        let mut elemexprs = self.node.into_iter();
        if let Some(ei) = elemexprs.next() {
            let (is_tail, expr) = ei.either(|x| (false, x), |x| (true, x.unwrap()));
            Continue(
                Scoped::new(self.scope.clone(), expr),
                Some(Scoped::new(
                    self.scope,
                    ListDefCont::new(elemexprs, is_tail),
                )),
            )
        } else {
            Produce(List::default().into())
        }
    }
}

impl<FX> Continuation<FX> for Scoped<ListDefCont<FX>>
where
    FX: Effect,
{
    fn eval_from_value(self, v: Value) -> Step<Scoped<Wise<FX>>, Self> {
        let Scoped { scope, mut node } = self;

        if node.is_tail {
            Produce(node.build_list(Some(v)).into())
        } else {
            node.vals.push(v);

            if let Some(ei) = node.elemexprs.next() {
                let (is_tail, expr) = ei.either(|x| (false, x), |x| (true, x.unwrap()));
                node.is_tail = is_tail;
                Continue(
                    Scoped::new(scope.clone(), expr),
                    Some(Scoped::new(scope, node)),
                )
            } else {
                Produce(node.build_list(None).into())
            }
        }
    }
}
