use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::ExprProvider;

/// Function application, ie `f x`.
#[derive(Debug, derive_new::new)]
pub struct ApplicationExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    /// The target of application, ie `f` in `f x`.
    pub target: Box<XP::Expr<FX>>,

    /// The argument of application, ie `x` in `f x`.
    pub argument: Box<XP::Expr<FX>>,
}

impl<XP, FX> ApplicationExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> ApplicationExpr<XPD, FX>
    where
        XPD: ExprProvider,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
    {
        ApplicationExpr {
            target: Box::new(XPD::Expr::from(*self.target)),
            argument: Box::new(XPD::Expr::from(*self.argument)),
        }
    }
}

impl<XP, FX> Unparse for ApplicationExpr<XP, FX>
where
    XP: ExprProvider,
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
    XP: ExprProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        ApplicationExpr::new(self.target.clone(), self.argument.clone())
    }
}

impl<XP, FX> PartialEq for ApplicationExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target && self.argument == other.argument
    }
}
