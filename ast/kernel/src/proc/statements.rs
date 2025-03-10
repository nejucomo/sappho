use sappho_unparse::{Stream, Unparse};

use crate::proc::ProcExpr;
use crate::Recursion;

use self::Statements::*;

#[derive(Debug)]
pub enum Statements<R>
where
    R: Unparse,
{
    Return(Recursion<ProcExpr<R>>),
}

impl<R> Unparse for Statements<R>
where
    R: Unparse,
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
