//! Top-level expression type `Expr`, generic over effects [PureEffect](sappho_ast_core::PureEffect), [QueryEffect](sappho_ast_core::QueryEffect), or [ProcEffect](sappho_ast_core::ProcEffect).

use sappho_ast_core::{CoreExpr, FuncDef, Literal, ProcDef, QueryDef};
use sappho_ast_effect::Effect;

use crate::{AstRich, ListExpr};

/// The general top-level expression for all effects.
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Expr<FX>
where
    FX: Effect,
{
    #[from(Literal)]
    Core(CoreExpr<AstRich, FX>),

    // Extensions from Core:
    Func(FuncDef<AstRich>),
    Query(QueryDef<AstRich>),
    Proc(ProcDef<AstRich>),
    List(ListExpr<FX>),
}

mod parsing {
    use std::fmt;

    use sappho_ast_effect::Effect;
    use sappho_syntax_unparse::{Stream, Unparse};

    use crate::Expr;

    impl<FX> Unparse for Expr<FX>
    where
        FX: Effect,
    {
        fn unparse_into(&self, s: &mut Stream) {
            use Expr::*;

            match self {
                Core(x) => x.unparse_into(s),
                Func(x) => x.unparse_into(s),
                Query(x) => x.unparse_into(s),
                Proc(x) => x.unparse_into(s),
                List(x) => x.unparse_into(s),
            }
        }
    }

    impl<FX> fmt::Display for Expr<FX>
    where
        FX: Effect,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.unparse().fmt(f)
        }
    }
}

mod generic_transforms {
    use sappho_ast_effect::Effect;

    use crate::Expr;

    impl<FX, T> From<Box<T>> for Expr<FX>
    where
        FX: Effect,
        Expr<FX>: From<T>,
    {
        fn from(b: Box<T>) -> Self {
            Self::from(*b)
        }
    }
}

mod syntax_transforms {
    use sappho_ast_core::{ApplicationExpr, Literal, LookupExpr};
    use sappho_ast_effect::{Effect, PureEffect};
    use sappho_primval::PrimVal;
    use sappho_syntax_expr as syntax;
    use sappho_syntax_listform::ListForm;
    use sappho_syntax_universal::UniExpr;

    use crate::Expr;

    impl From<syntax::Expr> for Expr<PureEffect> {
        fn from(synexpr: syntax::Expr) -> Self {
            Self::from(syntax::Applications::from(synexpr))
        }
    }

    impl From<syntax::Applications> for Expr<PureEffect> {
        fn from(synapp: syntax::Applications) -> Self {
            synapp
                .la_fold(|richexpr, inner| ApplicationExpr::new(richexpr, Self::from(inner)).into())
        }
    }

    impl From<syntax::Lookups> for Expr<PureEffect> {
        fn from(synloo: syntax::Lookups) -> Self {
            synloo.la_fold(|rx, lookup| LookupExpr::new(rx, lookup).into())
        }
    }

    impl From<syntax::InnerExpr> for Expr<PureEffect> {
        fn from(syn: syntax::InnerExpr) -> Self {
            match syn {
                syntax::InnerExpr::Uni(x) => Self::from(x),
                syntax::InnerExpr::Parens(x) => Self::from(x),
                syntax::InnerExpr::ListExpr(x) => Self::from(x),
            }
        }
    }

    impl<FX> From<UniExpr> for Expr<FX>
    where
        FX: Effect,
    {
        fn from(syn: syntax::Base) -> Self {
            match syn {
                syntax::Base::PrimVal(x) => Self::from(x),
                syntax::Base::Deref(x) => Self::from(x),
            }
        }
    }

    impl<FX> From<PrimVal> for Expr<FX>
    where
        FX: Effect,
    {
        fn from(pval: PrimVal) -> Self {
            match pval {
                // TODO: Replace `Literal` with `PrimVal`
                PrimVal::Num(i) => Self::from(Literal::from(i)),
            }
        }
    }

    impl From<syntax::ListExpr> for Expr<PureEffect> {
        fn from(value: syntax::ListExpr) -> Self {
            Expr::List(
                ListForm::try_from_iter(value.into_iter().map(|ei| {
                    ei.map_left(Self::from)
                        .map_right(Self::from)
                        .map_right(Box::new)
                }))
                .unwrap(),
            )
        }
    }
}

mod inner_transforms {
    use sappho_ast_core::{ApplicationExpr, CoreExpr, LetExpr, LookupExpr, MatchExpr, ObjectDef};
    use sappho_ast_effect::Effect;
    use sappho_attrs::Attrs;
    use sappho_syntax_idstore::ArcId;

    use crate::{AstRich, Expr};

    macro_rules! from_via_core_expr {
        ( $t:ty) => {
            impl<FX> From<$t> for Expr<FX>
            where
                FX: Effect,
            {
                fn from(value: $t) -> Self {
                    Self::from(CoreExpr::from(value))
                }
            }
        };
    }

    // Directly from `CoreExpr`:
    // from_via_core_expr!(Literal);
    from_via_core_expr!(ArcId);
    from_via_core_expr!(ObjectDef<AstRich, FX>);
    from_via_core_expr!(LetExpr<AstRich, FX>);
    from_via_core_expr!(MatchExpr<AstRich, FX>);
    from_via_core_expr!(ApplicationExpr<AstRich, FX>);
    from_via_core_expr!(LookupExpr<AstRich, FX>);

    // Indirectly through `CoreExpr`:
    from_via_core_expr!(Attrs<Expr<FX>>);
}

mod disorganized_transforms {
    use either::Either;
    use sappho_ast_effect::Effect;

    use crate::{Expr, ListExpr};

    // impl<FX, T> From<T> for Expr<FX>
    // where
    //     FX: Effect,
    //     CoreExpr<AstRich, FX>: From<T>,
    // {
    //     fn from(x: T) -> Self {
    //         Expr::Core(CoreExpr::from(x))
    //     }
    // }

    impl<FX> FromIterator<Either<Expr<FX>, Box<Expr<FX>>>> for Expr<FX>
    where
        FX: Effect,
    {
        fn from_iter<T>(iter: T) -> Self
        where
            T: IntoIterator<Item = Either<Expr<FX>, Box<Expr<FX>>>>,
        {
            Expr::List(ListExpr::try_from_iter(iter).unwrap())
        }
    }
}
