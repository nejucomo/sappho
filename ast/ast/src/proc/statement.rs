use sappho_ast_effect::ProcEffect;
use sappho_unparse::{Stream, Unparse};

use crate::CoreExpr;

#[derive(Clone, Debug, PartialEq)]
pub enum Statements {
    Return(Box<CoreExpr<ProcEffect>>),
}

impl Unparse for Statements {
    fn unparse_into(&self, s: &mut Stream) {
        use Statements::*;

        match self {
            Return(x) => {
                s.write("return ");
                s.write(x);
                s.write(";");
            }
        }
    }
}
