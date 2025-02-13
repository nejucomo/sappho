use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr};

/// Function application, ie `f x`.
#[derive(Debug, derive_new::new)]
pub struct ApplicationExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    /// The target of application, ie `f` in `f x`.
    pub target: BoxExpr<XP, FX>,

    /// The argument of application, ie `x` in `f x`.
    pub argument: BoxExpr<XP, FX>,
}

impl<XPD, XPS, FX> AstTransformInto<ApplicationExpr<XPD, FX>> for ApplicationExpr<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> ApplicationExpr<XPD, FX> {
        ApplicationExpr {
            target: self.target.ast_transform(),
            argument: self.argument.ast_transform(),
        }
    }
}

impl<XP, FX> Unparse for ApplicationExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{
            Brackets::Parens,
            Break::{Opt, OptSpace},
        };

        s.bracketed(Parens, |subs| {
            subs.write(&Opt);
            subs.write(&self.target);
            subs.write(&OptSpace);
            subs.write(&self.argument);
        });
    }
}

impl<XP, FX> Clone for ApplicationExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        ApplicationExpr::new(self.target.clone(), self.argument.clone())
    }
}

impl<XP, FX> PartialEq for ApplicationExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target && self.argument == other.argument
    }
}
