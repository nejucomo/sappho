use crate::ScopeRef;
use crate::ValRef;
use sappho_east::{FuncClause, Pattern, PureExpr};
use std::rc::Rc;

pub struct Func {
    binding: Pattern,
    body: Rc<PureExpr>,
    defscope: ScopeRef,
}

impl Func {
    pub fn new(fc: &FuncClause, defscope: &ScopeRef) -> Self {
        Func {
            binding: fc.binding.clone(),
            body: fc.body.clone(),
            defscope: defscope.clone(),
        }
    }

    // FIXME: introduce generic `Thunk` for eval.
    pub fn bind_arg(&self, arg: &ValRef) -> (&PureExpr, ScopeRef) {
        let callscope = self.defscope.extend(&self.binding, arg.clone());
        (&self.body, callscope)
    }
}
