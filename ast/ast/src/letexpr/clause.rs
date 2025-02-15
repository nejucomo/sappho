use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{CoreExpr, Pattern};

#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct LetClause<FX>
where
    FX: Effect,
{
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: Box<CoreExpr<FX>>,
}

impl<FX> Unparse for LetClause<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("let ");
        s.write(&self.binding);
        s.write(" = ");
        s.write(&self.bindexpr);
    }
}
