use sappho_ast_effect::ProcEffect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr};

#[derive(Debug)]
pub enum Statements<XP>
where
    XP: AstProvider,
{
    Return(BoxExpr<XP, ProcEffect>),
}

impl<XPS, XPD> AstTransformInto<Statements<XPD>> for Statements<XPS>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<ProcEffect>: AstTransformInto<XPD::Expr<ProcEffect>>,
{
    fn ast_transform(self) -> Statements<XPD> {
        use Statements::*;

        match self {
            Return(x) => Return(x.ast_transform()),
        }
    }
}

impl<XP> Unparse for Statements<XP>
where
    XP: AstProvider,
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
    XP: AstProvider,
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
    XP: AstProvider,
{
    fn eq(&self, other: &Self) -> bool {
        use Statements::*;

        match (self, other) {
            (Return(a), Return(b)) => a == b,
        }
    }
}
