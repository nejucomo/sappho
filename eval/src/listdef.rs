use either::Either;
use sappho_east::{BoxWise, ListDef, Wise};
use sappho_effect::Effect;
use sappho_list::List;
use sappho_listform::ListFormRefIter;
use sappho_value::{Scope, Valuable as _, Value};

use crate::contiter::{ContIter, IterContinuation};
use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;

pub(crate) type ContListDef<'x, FX> =
    ContIter<ListBuilder, ListFormRefIter<'x, Wise<FX>, BoxWise<FX>>, bool>;

/// # TODO: This looks suspiciously similar to [ListForm]
#[derive(Debug, Default)]
pub(crate) struct ListBuilder {
    items: Vec<Value>,
    optail: Option<List<Value>>,
}

impl<'s, 'x, FX> Eval<&'s Scope, ExprStep<'x, FX>> for &'x ListDef<FX>
where
    FX: Effect,
{
    fn eval(self, _: &'s Scope) -> ExprStep<'x, FX> {
        ListBuilder::default().eval_via_iter(self.iter())
    }
}

impl<'x, FX> IterContinuation<'x, FX, ListFormRefIter<'x, Wise<FX>, BoxWise<FX>>, bool>
    for ListBuilder
where
    FX: Effect,
{
    fn split_item(item: Either<&'x Wise<FX>, &'x BoxWise<FX>>) -> (bool, &'x Wise<FX>) {
        item.either(|w| (false, w), |bw| (true, bw))
    }
}

impl From<ListBuilder> for Value {
    fn from(ListBuilder { items, optail }: ListBuilder) -> Self {
        let tail = optail.unwrap_or_default();
        let list = items.into_iter().rev().fold(tail, |t, v| t.prepend(v));
        Value::from(list)
    }
}

impl Extend<(bool, Value)> for ListBuilder {
    fn extend<T: IntoIterator<Item = (bool, Value)>>(&mut self, iter: T) {
        for (is_tail, v) in iter {
            if is_tail {
                let tail = v
                    .as_list()
                    .expect("TODO: propagate user-space errors")
                    .clone();

                assert!(self.optail.replace(tail).is_none());
            } else {
                self.items.push(v);
            }
        }
    }
}
