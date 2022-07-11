use crate::{BindFailure, GenThunk, ScopeRef, ValRef};
use sappho_east::{FuncClause, Pattern, PureEffects, PureExpr};
use std::fmt;

#[derive(Debug)]
pub struct Func {
    binding: Pattern,
    body: PureExpr,
    defscope: ScopeRef,
}

impl Func {
    pub fn new(fc: &FuncClause, defscope: &ScopeRef) -> Self {
        Func {
            binding: fc.binding.clone(),
            body: (*fc.body).clone(),
            defscope: defscope.clone(),
        }
    }

    pub fn bind_arg(&self, arg: &ValRef) -> Result<GenThunk<PureEffects>, BindFailure> {
        let callscope = self.defscope.declare_then_bind(&self.binding, arg)?;
        Ok(GenThunk::new(self.body.clone(), callscope))
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn ")?;
        self.binding.fmt(f)?;
        write!(f, " -> ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
