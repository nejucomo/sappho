use std::collections::BTreeMap;

use sappho_attrs::Attrs;
use sappho_continuation::{Continuation, ContinueStep};
use sappho_east::{Expr, ObjectDef, Wise};
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_value::{ObjectVal, Scope, Value};

use crate::eval::EvalResult;
use crate::expr::ContExpr;
use crate::exprstep::ExprStep;
use crate::itercont::IterContinuation;
use crate::wrapper::Ev;

impl<'s, 'x, FX> Continuation<&'s Scope, EvalResult<Value>, Ev<&'x Expr<FX>>, ContExpr<'x, FX>>
    for Ev<&'x ObjectDef<FX>>
where
    FX: Effect + 'x,
{
    fn continue_with(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        let (optf, optq, optp, attrexprs) = self.as_refs().into();
        let objval = ObjectVal::new(
            optf.cloned(),
            optq.cloned(),
            optp.cloned(),
            Attrs::default(),
        );
        let iter = attrexprs.into_iter();
        ContinueStep::new(iter, Ev(objval)).into()
    }
}

impl<'s, FX> IterContinuation<'s, FX> for ObjectVal
where
    FX: Effect + 's,
{
    type Iter = <BTreeMap<RcId, &'s Wise<FX>> as IntoIterator>::IntoIter;
    type Aux = RcId;

    fn eval_next_from_iter(self, iter: Self::Iter) -> EvalStep<'s, FX> {
        todo!()
    }
}
