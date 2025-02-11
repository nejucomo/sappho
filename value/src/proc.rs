use crate::{GenThunk, ScopeRef};
use sappho_ast_core::{EffectExpr, ProcEffect};
use sappho_ast_reduced::{ObjectDef, ProcClause, ProcExpr};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_new::new)]
pub struct Proc {
    pdef: ProcClause,
    defscope: ScopeRef,
}

impl Proc {
    pub fn as_thunk(&self) -> GenThunk<ProcEffect> {
        // FIXME: This is ugly: GenThunk requires an `Expr` so we synthsize `!proc { ... }` around
        // the proc definition.
        GenThunk::new(
            ProcExpr::new(EffectExpr::new(
                ProcEffect::Invoke,
                Box::new(ProcExpr::new(ObjectDef::new_proc(self.pdef.clone()))),
            )),
            self.defscope.clone(),
        )
    }
}

impl Unparse for Proc {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.pdef);
    }
}
