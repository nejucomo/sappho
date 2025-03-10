use sappho_unparse::Unparse;

use crate::proc::Statements;

#[derive(Debug, derive_more::From)]
pub struct ProcDef<R>
where
    R: Unparse,
{
    body: Statements<R>,
}

impl<R> Unparse for ProcDef<R>
where
    R: Unparse,
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
