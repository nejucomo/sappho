use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, ProcExpr};

#[derive(Clone, Debug, PartialEq)]
pub enum Statements<XP>
where
    XP: AstProvider,
{
    Return(ProcExpr<XP>),
}

// impl<XP> Statements<XP>
// where
//     XP: AstProvider,
// {
//     pub fn transform_into<XPD>(self) -> Statements<XPD>
//     where
//         XPD: AstProvider,
//         XPD::Expr<ProcEffect>: From<XP::Expr<ProcEffect>>,
//     {
//         use Statements::*;

//         match self {
//             Return(x) => Return(Box::new(XPD::Expr::from(*x))),
//         }
//     }
// }

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
