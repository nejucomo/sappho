use sappho_unparse::{Stream, Unparse};

use crate::proc::ProcExpr;
use crate::{Expression, Recursion};

use self::Statements::*;

#[derive(Debug)]
pub enum Statements<X>
where
    X: Expression,
{
    Return(Recursion<ProcExpr<X>>),
}

impl<X> Unparse for Statements<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Return(x) => {
                s.write("return ");
                s.write(x);
                s.write(";");
            }
        }
    }
}
