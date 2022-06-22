use crate::{BindFailure, GenThunk, ScopeRef, ValRef};
use sappho_east::{FuncClause, Pattern, PureEffects, PureExpr};
use std::rc::Rc;

pub struct Func {
    binding: Rc<Pattern>,
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

    pub fn bind_arg(&self, arg: &ValRef) -> Result<GenThunk<PureEffects>, BindFailure> {
        let callscope = self.defscope.bind(&self.binding, arg)?;
        Ok(GenThunk::new(&self.body, callscope))
    }
}
