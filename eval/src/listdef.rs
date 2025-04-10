use derive_more::From;
use either::Either;
use sappho_east::{BoxWise, ListDef, Wise};
use sappho_effect::Effect;
use sappho_list::List;
use sappho_listform::ListFormIter;
use sappho_value::{Scope, Value};

use crate::continuation::EvalStep;
use crate::itercont::{IterCont, IterContinuation};
use crate::step::Step;

impl<FX> EvalStep<FX> for ListDef<FX>
where
    FX: Effect,
{
    type Continuation<'a> = ListDefCont<'a, FX>;

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation<'a>> {
        Inner::default()
            .eval_from_iter(scope.clone(), self.iter())
            .cont_from()
    }
}

#[derive(Debug, From)]
pub(crate) struct ListDefCont<'a, FX>
where
    FX: Effect,
{
    ic: IterCont<'a, FX, Inner, ListFormIter<std::slice::Iter<'a, Wise<FX>>, &'a BoxWise<FX>>>,
}

#[derive(Debug, Default)]
struct Inner {
    items: Vec<Value>,
    optail: Option<List<Value>>,
}

impl<'a, FX> IterContinuation<'a, FX> for Inner
where
    FX: Effect + 'a,
{
    type Item = Either<&'a Wise<FX>, &'a BoxWise<FX>>;

    /// A `bool` representing `the value is the tail`
    type Key = bool;

    fn unpack_item(item: Self::Item) -> (Self::Key, &'a Wise<FX>) {
        item.either(|w| (false, w), |bw| (true, &**bw))
    }

    fn eval_from_iter_done(self) -> Value {
        todo!()
    }

    fn extend_with_key_value(&mut self, is_tail: bool, v: Value) {
        if is_tail {
            let old = self.optail.replace(v.try_into().expect(
                "Expected tail to be a list. TODO: Add a way to propagate user-space exceptions.",
            ));

            assert!(
                old.is_none(),
                "`ListForm::into_iter/iter` postcondition failure."
            );
        } else {
            self.items.push(v);
        }
    }
}
