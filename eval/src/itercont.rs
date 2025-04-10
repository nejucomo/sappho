use std::marker::PhantomData;

use sappho_east::Wise;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::continuation::Continuation;
use crate::step::Step::{self, Continue, Produce};

pub(crate) trait IterContinuation<'a, FX>: Sized
where
    FX: Effect,
{
    type Key;
    type Item;

    fn eval_from_iter<I>(
        self,
        scope: Scope,
        init: I,
    ) -> Step<'a, FX, IterCont<'a, FX, Self, I::IntoIter>>
    where
        I: IntoIterator<Item = Self::Item>,
    {
        let mut it = init.into_iter();
        if let Some(item) = it.next() {
            let (key, expr) = Self::unpack_item(item);
            Continue(
                scope.into(),
                expr,
                Some(IterCont {
                    ic: self,
                    it,
                    key,
                    ph: PhantomData,
                }),
            )
        } else {
            Produce(self.eval_from_iter_done())
        }
    }

    fn unpack_item(item: Self::Item) -> (Self::Key, &'a Wise<FX>);

    fn eval_from_iter_done(self) -> Value;

    fn extend_with_key_value(&mut self, k: Self::Key, v: Value);
}

#[derive(Debug)]
pub(crate) struct IterCont<'a, FX, C, I>
where
    FX: Effect,
    C: IterContinuation<'a, FX>,
    I: Iterator<Item = C::Item>,
{
    ic: C,
    it: I,
    key: C::Key,
    ph: PhantomData<&'a FX>,
}

impl<'a, FX, C, I> Continuation<'a, FX> for IterCont<'a, FX, C, I>
where
    FX: Effect,
    C: IterContinuation<'a, FX>,
    I: Iterator<Item = C::Item>,
{
    fn eval_from_value(self, scope: Scope, v: Value) -> Step<'a, FX, Self> {
        let IterCont {
            mut ic,
            it,
            key,
            ph: _,
        } = self;
        ic.extend_with_key_value(key, v);
        ic.eval_from_iter(scope, it)
    }
}
