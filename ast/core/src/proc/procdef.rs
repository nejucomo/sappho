use crate::{AstProvider, Statements};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_more::From, derive_more::Into)]
pub struct ProcDef<XP>(Statements<XP>)
where
    XP: AstProvider;

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
