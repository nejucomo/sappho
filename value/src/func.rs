use crate::{BindFailure, GenThunk, ScopeRef, ValRef};
use sappho_east::{FuncClause, Pattern, PureEffects, PureExpr};
use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};

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

impl DisplayDepth for Func {
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        write!(f, "fn ")?;
        self.binding.fmt_depth(f, depth)?;
        write!(f, " -> ")?;
        self.body.fmt_depth(f, depth)?;
        Ok(())
    }
}
