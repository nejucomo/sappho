use crate::{GenThunk, ScopeRef};
use sappho_ast_core::{EffectExpr, ProcEffects};
use sappho_ast_reduced::{ObjectDef, ProcClause, ProcExpr};
use sappho_legible::{IntoNode, Node};

#[derive(Debug, derive_new::new)]
pub struct Proc {
    pdef: ProcClause,
    defscope: ScopeRef,
}

impl Proc {
    pub fn as_thunk(&self) -> GenThunk<ProcEffects> {
        // FIXME: This is ugly: GenThunk requires an `Expr` so we synthsize `!proc { ... }` around
        // the proc definition.
        GenThunk::new(
            ProcExpr::new(EffectExpr::new(
                ProcEffects::Invoke,
                Box::new(ProcExpr::new(ObjectDef::new_proc(self.pdef.clone()))),
            )),
            self.defscope.clone(),
        )
    }
}

impl IntoNode for &Proc {
    fn into_node(self) -> Node {
        self.pdef.into_node()
    }
}
