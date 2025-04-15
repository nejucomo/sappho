use sappho_attrs::Attrs;
use sappho_east::{ObjectDef, Wise};
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_value::{FuncVal, ObjectVal, ProcVal, QueryVal, Scope, Value};

use crate::contiter::{continue_eval_via_iter, ContIter};
use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;

pub(crate) type ContObjectDef<'x, FX> = ContIter<ObjectBuilder, ExprAttrsIter<'x, FX>, RcId>;

type ExprAttrsIter<'x, FX> = <Attrs<&'x Wise<FX>> as IntoIterator>::IntoIter;

impl<'s, 'x, FX> Eval<&'s Scope, ExprStep<'x, FX>> for &'x ObjectDef<FX>
where
    FX: Effect,
{
    fn eval(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        let (fdef, qdef, pdef, exprattrs) = self.as_refs().into();

        let oval = ObjectBuilder {
            f: fdef.cloned().map(|def| scope.clone_wrap(def)),
            q: qdef.cloned().map(|def| scope.clone_wrap(def)),
            p: pdef.cloned().map(|def| scope.clone_wrap(def)),
            a: Attrs::default(),
        };

        continue_eval_via_iter(oval, exprattrs.into_iter())
    }
}

/// We implement this new type rather than providing a mutable interface for `ObjectVal` to help ensure `Value` is immutable (except for explicit mutables)
#[derive(Debug)]
pub(crate) struct ObjectBuilder {
    f: Option<FuncVal>,
    q: Option<QueryVal>,
    p: Option<ProcVal>,
    a: Attrs<Value>,
}

impl Extend<(RcId, Value)> for ObjectBuilder {
    fn extend<T: IntoIterator<Item = (RcId, Value)>>(&mut self, iter: T) {
        for (rcid, value) in iter {
            self.a
                .define(rcid, value)
                .expect("Parsing failed to ensure unique attr names")
        }
    }
}

impl From<ObjectBuilder> for Value {
    fn from(ObjectBuilder { f, q, p, a }: ObjectBuilder) -> Self {
        ObjectVal::new_from_parts(f, q, p, a).into()
    }
}
