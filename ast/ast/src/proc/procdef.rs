use sappho_unparse::{Stream, Unparse};

use crate::Statements;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ProcDef(Statements);

impl Unparse for ProcDef {
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
