use crate::eval::Eval;
use crate::scope::ScopeRef;
use crate::{Result, ValRef};
use sappho_east::{FuncClause, Pattern, PureExpr};
use std::rc::Rc;

pub struct Func {
    binding: Pattern,
    body: Rc<PureExpr>,
    defscope: ScopeRef,
}

impl Func {
    pub(crate) fn new(fc: &FuncClause, defscope: ScopeRef) -> Self {
        Func {
            binding: fc.binding.clone(),
            body: fc.body.clone(),
            defscope,
        }
    }

    pub fn apply(&self, arg: &ValRef) -> Result<ValRef> {
        let callscope = self.defscope.extend(&self.binding, arg.clone());
        self.body.eval(callscope)
    }
}
