use std::collections::BTreeMap;

use derive_more::From;
use sappho_attrs::Attrs;
use sappho_east::{ObjectDef, Wise};
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_value::{FuncVal, ObjectVal, Scope, Value};

use crate::continuation::EvalStep;
use crate::itercont::{IterCont, IterContinuation};
use crate::step::Step;

#[derive(Debug, From)]
pub(crate) struct ObjDefCont<'a, FX>(
    IterCont<'a, FX, ObjectVal, <BTreeMap<RcId, &'a Wise<FX>> as IntoIterator>::IntoIter>,
)
where
    FX: Effect;

impl<'a, FX> IterContinuation<'a, FX> for ObjectVal
where
    FX: Effect,
{
    type Key = RcId;

    fn eval_from_iter_done(self) -> Value {
        Value::from(self)
    }

    fn extend_with_key_value(&mut self, k: Self::Key, v: Value) {
        self.attrs_mut()
            .define(k, v)
            .expect("TODO: propagate user-space errors")
    }
}

impl<FX> EvalStep<FX> for ObjectDef<FX>
where
    FX: Effect,
{
    type Continuation<'a> = ObjDefCont<'a, FX>;

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation<'a>> {
        let (optf, optq, optp, exprattrs) = self.as_refs().into();
        ObjectVal::new(
            optf.map(|fdef| FuncVal::new(scope.clone(), fdef.clone())),
            optq.cloned(),
            optp.cloned(),
            Attrs::default(),
        )
        .eval_from_iter(scope.clone(), exprattrs)
        .cont_from()
    }
}
