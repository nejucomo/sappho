use crate::{AstProvider, AstTransformInto, Statements};
use sappho_ast_effect::ProcEffect;
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_more::From)]
pub struct ProcDef<XP>(Statements<XP>)
where
    XP: AstProvider;

impl<XPS, XPD> AstTransformInto<ProcDef<XPD>> for ProcDef<XPS>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<ProcEffect>: AstTransformInto<XPD::Expr<ProcEffect>>,
{
    fn ast_transform(self) -> ProcDef<XPD> {
        ProcDef(self.0.ast_transform())
    }
}

impl<XP> Unparse for ProcDef<XP>
where
    XP: AstProvider,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Brackets::Squiggle;
        use sappho_unparse::Break;

        s.write("proc ");
        s.bracketed(Squiggle, |subs| {
            subs.write(&Break::Mandatory);
            subs.write(&self.0);
        });
    }
}

impl<XP> Clone for ProcDef<XP>
where
    XP: AstProvider,
{
    fn clone(&self) -> Self {
        ProcDef::from(self.0.clone())
    }
}

impl<XP> PartialEq for ProcDef<XP>
where
    XP: AstProvider,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
