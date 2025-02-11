use crate::{AstProvider, Statements};
use sappho_ast_effect::ProcEffect;
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_more::From)]
pub struct ProcDef<XP>(Statements<XP>)
where
    XP: AstProvider;

impl<XP> ProcDef<XP>
where
    XP: AstProvider,
{
    pub fn transform_into<XPD>(self) -> ProcDef<XPD>
    where
        XPD: AstProvider,
        XPD::Expr<ProcEffect>: From<XP::Expr<ProcEffect>>,
    {
        ProcDef(self.0.transform_into())
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
