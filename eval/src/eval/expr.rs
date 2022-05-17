use super::Eval;
use crate::scope::ScopeRef;
use crate::{Result, ValRef};
use saplang_east::GenExpr;

impl<FX> Eval for GenExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        use GenExpr::*;

        match &self {
            Universal(x) => x.eval(scope),
            Common(x) => x.eval(scope),
            Recursive(x) => x.eval(scope),
            Effect(x) => x.eval(scope),
        }
    }
}
