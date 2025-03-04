use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::{AstProvider, CmtExpr};

#[derive(Clone, Debug, derive_more::From, derive_more::Into, PartialEq)]
pub struct ParensExpr<XP, FX>(CmtExpr<XP, FX>)
where
    XP: AstProvider,
    FX: Effect;

impl<XP, FX> Unparse for ParensExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        s.write("(");
        s.write(&self.0);
        s.write(")");
    }
}
