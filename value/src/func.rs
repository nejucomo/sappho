use crate::{BindFailure, GenThunk, ScopeRef, ValRef};
use sappho_ast_core::PureEffects;
use sappho_ast_reduced::{FuncClause, Pattern, PureExpr};
use sappho_legible::{HeadAndTail, IntoNode, Node};

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

impl IntoNode for &Func {
    fn into_node(self) -> Node {
        HeadAndTail {
            head: ("fn ", &self.binding, " ->"),
            sep: " ",
            tail: &self.body,
        }
        .into_node()
    }
}
