use sappho_attrs::Attrs;
use sappho_east::ObjectDef;
use sappho_effect::Effect;
use sappho_value::{FuncVal, ObjectVal, ProcVal, QueryVal, Scope};

use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;

impl<'s, 'x, FX> Eval<&'s Scope, ExprStep<'x, FX>> for &'x ObjectDef<FX>
where
    FX: Effect,
{
    fn eval(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        let (fdef, qdef, pdef, exprattrs) = self.as_refs().into();

        let fval = fdef.map(|def| FuncVal::new(scope.clone(), def));
        let qval = qdef.map(|def| QueryVal::new(scope.clone(), def));
        let pval = pdef.map(|def| ProcVal::new(scope, def));

        let oval = ObjectVal::new_from_parts(fval, qval, pval, Attrs::default());
        oval.eval(exprattrs)
    }
}
