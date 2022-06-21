use crate::{GenThunk, ScopeRef, ValRef};
use sappho_east::{FuncClause, Pattern, PureEffects, PureExpr};
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

    pub fn bind_arg(&self, arg: &ValRef) -> GenThunk<PureEffects> {
        let callscope = self.defscope.extend(&self.binding, arg.clone());
        GenThunk::new(&self.body, callscope)
    }
}
