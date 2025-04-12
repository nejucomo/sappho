use std::collections::BTreeMap;

use sappho_attrs::Attrs;
use sappho_east::{ObjectDef, Wise};
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_value::{ObjectVal, Scope};

use crate::evalstep::EvalStep;
use crate::expr::ExprEvalNext;
use crate::itercont::IterContinuation;

impl<'s, FX> ExprEvalNext<'s, FX> for &'s ObjectDef<FX>
where
    FX: Effect,
{
    fn expr_eval_next(self, scope: &Scope) -> EvalStep<'s, FX> {
        let (optf, optq, optp, attrexprs) = self.as_refs().into();
        let objval = ObjectVal::new(
            optf.cloned(),
            optq.cloned(),
            optp.cloned(),
            Attrs::default(),
        );
        let iter = attrexprs.into_iter();
        objval.eval_next_from_iter(iter)
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
