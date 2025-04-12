use sappho_east::{Expr, ObjectDef, Wise};
use sappho_effect::Effect;
use sappho_value::Value;

use crate::continuation::ContinueEval;
use crate::continuation::Step::{self, *};

impl<'s, FX> ContinueEval<Value, &'s Wise<FX>, ExprCont<'s, FX>> for &'s Expr<FX>
where
    FX: Effect,
{
    fn continue_eval(self) -> Step<Value, &'s Wise<FX>, ExprCont<'s, FX>> {
        use Expr::*;

        match self {
            Prim(x) => Step::conclude(x),
            Ref(x) => todo!("fixme scoping {x:#?}"),,
            ObjectDef(x) => x,
            ListDef(x) => x,
            Let(x) => x,
            Match(x) => x,
            Application(x) => x,
            Lookup(x) => x,
            Interaction(x) => x,
        }
    }
}
