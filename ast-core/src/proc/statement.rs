use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq)]
pub enum Statements<ProcExpr> {
    Return(Box<ProcExpr>),
}

impl<X> Statements<X> {
    pub fn transform_into<XD>(self) -> Statements<XD>
    where
        XD: From<X>,
    {
        use Statements::*;

        match self {
            Return(x) => Return(Box::new(XD::from(*x))),
        }
    }
}

impl<X> Unparse for Statements<X>
where
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use Statements::*;

        match self {
            Return(x) => {
                s.write("return ");
                s.write(x);
            }
        }
    }
}
