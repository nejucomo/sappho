use derive_more::From;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, Statements};

#[derive(Clone, Debug, PartialEq, From)]
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
