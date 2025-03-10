use sappho_unparse::Unparse;

use crate::proc::Statements;
use crate::Expression;

#[derive(Debug, derive_more::From)]
pub struct ProcDef<X>
where
    X: Expression,
{
    body: Statements<X>,
}

impl<X> Unparse for ProcDef<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        use sappho_unparse::Brackets::Squiggle;
        use sappho_unparse::Break;

        s.write("proc ");
        s.bracketed(Squiggle, |subs| {
            subs.write(&Break::Mandatory);
            subs.write(&self.body);
        });
    }
}
