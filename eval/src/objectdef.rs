use derive_new::new;
use sappho_attrs::Attrs;
use sappho_east::{FuncDef, ObjectDef, ProcDef, QueryDef, Wise};
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_value::{ObjectVal, Scope, Value};

use crate::continuation::{Continuation, EvalStep};
use crate::scoped::Scoped;
use crate::step::Step::{self, Continue, Produce};

#[derive(Debug, new)]
pub(crate) struct ObjDefCont<FX>
where
    FX: Effect,
{
    // Pre-defined parsing outputs:
    scope: Scope,
    optfunc: Option<FuncDef>,
    optquery: Option<QueryDef>,
    optproc: Option<ProcDef>,

    // Pending runtime state:
    #[new(default)]
    attrvals: Attrs<Value>,
    attrname: Option<RcId>,
    expritems: <Attrs<Wise<FX>> as IntoIterator>::IntoIter,
}

impl<FX> From<ObjDefCont<FX>> for Value
where
    FX: Effect,
{
    fn from(mut value: ObjDefCont<FX>) -> Self {
        assert!(value.attrname.take().is_none());

        let ObjDefCont {
            scope,
            optfunc,
            optquery,
            optproc,
            attrvals,
            ..
        } = value;
        ObjectVal::new(scope, (optfunc, optquery, optproc, attrvals)).into()
    }
}

impl<FX> EvalStep<FX> for Scoped<ObjectDef<FX>>
where
    FX: Effect,
{
    type Continuation = ObjDefCont<FX>;

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation> {
        let (optf, optq, optp, exprattrs) = self.node.unwrap().into();
        let mut expritems = exprattrs.into_iter();
        if let Some((rcid, expr)) = expritems.next() {
            Continue(
                Scoped::new(self.scope.clone(), expr),
                Some(ObjDefCont::new(
                    self.scope,
                    optf,
                    optq,
                    optp,
                    Some(rcid),
                    expritems,
                )),
            )
        } else {
            Produce(ObjectVal::new(self.scope, (optf, optq, optp, Attrs::default())).into())
        }
    }
}

impl<FX> Continuation<FX> for ObjDefCont<FX>
where
    FX: Effect,
{
    fn eval_from_value(mut self, v: Value) -> Step<'a, FX, Self> {
        self.attrvals
            .define(self.attrname.take().unwrap(), v)
            .unwrap();

        if let Some((rcid, expr)) = self.expritems.next() {
            self.attrname = Some(rcid);
            Continue(Scoped::new(self.scope.clone(), expr), Some(self))
        } else {
            Produce(self.into())
        }
    }
}
