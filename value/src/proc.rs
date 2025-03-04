use crate::{GenThunk, ScopeRef};
use sappho_ast_core::{CmtExpr, EffectExpr, ProcDef};
use sappho_ast_effect::ProcEffect;
use sappho_ast_reduced::{AstRed, Expr};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_new::new)]
pub struct Proc {
    pdef: ProcDef<AstRed>,
    defscope: ScopeRef,
}

impl Proc {
    // TODO: Can we remove this method?
    pub fn as_thunk(&self) -> GenThunk<ProcEffect> {
        // FIXME: This is ugly: GenThunk requires an `Expr` so we synthsize `!proc { ... }` around
        // the proc definition.
        GenThunk::new(
            CmtExpr::from((
                None,
                Expr::new(EffectExpr::new(
                    ProcEffect::Invoke,
                    CmtExpr::from(self.pdef.clone()),
                )),
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
