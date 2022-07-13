use crate::{BindFailure, GenThunk, ScopeRef, ValRef};
use sappho_east::{FuncClause, Pattern, PureEffects, PureExpr};
use sappho_unparse::{Unparse, Stream};

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

impl Unparse for Func {
    fn unparse_into(&self, s: &mut Stream) {
        write!(f, "fn ")?;
        self.binding.unparse(f, depth)?;
        write!(f, " -> ")?;
        self.body.unparse(f, depth)?;
        Ok(())
    }
}
