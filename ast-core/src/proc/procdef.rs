use crate::Statements;
use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ProcDef<ProcExpr>(Statements<ProcExpr>);

impl<X> ProcDef<X> {
    pub fn transform_into<XD>(self) -> ProcDef<XD>
    where
        XD: From<X>,
    {
        ProcDef(self.0.transform_into())
    }
}

impl<X> Unparse for ProcDef<X>
where
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Brackets::Squiggle;

        s.write("proc ");
        s.bracketed(Squiggle, |subs| {
            subs.write(&self.0);
        });
    }
}
