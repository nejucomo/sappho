use derive_more::From;
use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;
use sappho_with_source::WithSource;

use crate::AstProvider;

/// WIth Source Expression is the top-level recursion entrypoint for AST expressions
#[derive(Clone, Debug, From, PartialEq)]
pub struct Wise<XP, FX>(WithSource<XP::Expr<FX>>)
where
    XP: AstProvider,
    FX: Effect;

impl<XP, FX> Unparse for Wise<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}

mod syntax_conversions {
    use sappho_ast_effect::Effect;
    use sappho_syntax as syntax;
    use sappho_with_source::WithSource;

    use crate::{AstProvider, Wise};

    impl<XP, FX> From<syntax::Wise<FX>> for Wise<XP, FX>
    where
        XP: AstProvider,
        FX: Effect,
        XP::Expr<FX>: From<syntax::Expr<FX>>,
    {
        fn from(value: syntax::Wise<FX>) -> Self {
            let synws = WithSource::<syntax::Expr<FX>>::from(value);
            Self(synws.map(XP::Expr::<FX>::from))
        }
    }
}
