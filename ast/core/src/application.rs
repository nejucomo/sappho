use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::CoreExpr;

/// Function application, ie `f x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct ApplicationExpr<FX>
where
    FX: Effect,
{
    /// The target of application, ie `f` in `f x`.
    pub target: Box<CoreExpr<FX>>,

    /// The argument of application, ie `x` in `f x`.
    pub argument: Box<CoreExpr<FX>>,
}

impl<FX> Unparse for ApplicationExpr<FX>
where
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
