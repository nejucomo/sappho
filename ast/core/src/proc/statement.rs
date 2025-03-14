use sappho_ast_effect::ProcEffect;
use sappho_unparse::{Stream, Unparse};

use crate::AstProvider;

#[derive(Debug)]
pub enum Statements<XP>
where
    XP: AstProvider<FX>,
{
    Return(Box<XP::Expr<ProcEffect>>),
}

impl<XP> Statements<XP>
where
    XP: AstProvider<FX>,
{
    pub fn transform_into<XPD>(self) -> Statements<XPD>
    where
        XPD: AstProvider<FX>,
        XPD::Expr<ProcEffect>: From<XP::Expr<ProcEffect>>,
    {
        use Statements::*;

        match self {
            Return(x) => Return(Box::new(XPD::Expr::from(*x))),
        }
    }
}

impl<XP> Unparse for Statements<XP>
where
    XP: AstProvider<FX>,
{
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

impl<XP> Clone for Statements<XP>
where
    XP: AstProvider<FX>,
{
    fn clone(&self) -> Self {
        use Statements::*;

        match self {
            Return(x) => Return(x.clone()),
        }
    }
}

impl<XP> PartialEq for Statements<XP>
where
    XP: AstProvider<FX>,
{
    fn eq(&self, other: &Self) -> bool {
        use Statements::*;

        match (self, other) {
            (Return(a), Return(b)) => a == b,
        }
    }
}
