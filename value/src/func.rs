use crate::{BindFailure, GenThunk, ScopeRef, ValRef};
use sappho_ast_reduced::{FuncClause, Pattern, PureExpr};
use sappho_astcore::PureEffects;
use sappho_unparse::{Stream, Unparse};

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
        s.write("fn ");
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.body);
    }
}
