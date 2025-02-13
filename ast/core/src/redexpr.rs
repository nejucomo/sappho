use derive_new::new;
use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::{AstRed, AstTransformInto, CoreExpr};

#[derive(Clone, Debug, PartialEq, new)]
pub struct RedExpr<FX>(CoreExpr<AstRed, FX>)
where
    FX: Effect;

impl<FX> AstTransformInto<RedExpr<FX>> for RedExpr<FX>
where
    FX: Effect,
{
    fn ast_transform(self) -> RedExpr<FX> {
        self
    }
}

impl<FX> AstTransformInto<CoreExpr<AstRed, FX>> for RedExpr<FX>
where
    FX: Effect,
{
    fn ast_transform(self) -> CoreExpr<AstRed, FX> {
        self.0
    }
}

impl<FX> Unparse for RedExpr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}
