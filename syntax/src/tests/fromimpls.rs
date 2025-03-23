use either::Either::Left;
use sappho_ast_effect::Effect;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_primval::{Num, PrimVal};
use sappho_source::{Source, SourceCodeRef};
use sappho_with_source::WithSource;

use crate::leftassoc::LeftAssoc;
use crate::{Applications, BoxWise, Confined, Expr, Interactions, Let, Lookups, Wise};

// Basic Entry points
macro_rules! impl_entrypoints {
    ( $s:ty => $t:ty ) => {
        impl<FX> From<i32> for $t
        where
            FX: Effect,
        {
            fn from(v: i32) -> Self {
                <$t>::from(<$s>::from(v))
            }
        }

        impl<FX> From<&'static str> for $t
        where
            FX: Effect,
        {
            fn from(v: &'static str) -> Self {
                <$t>::from(<$s>::from(v))
            }
        }

        impl<FX> From<()> for $t
        where
            FX: Effect,
        {
            fn from((): ()) -> Self {
                <$t>::from(<$s>::from(()))
            }
        }

        impl<FX, T, const K: usize> From<[T; K]> for $t
        where
            FX: Effect,
            Confined<FX>: From<[T; K]>,
        {
            fn from(v: [T; K]) -> Self {
                <$t>::from(<$s>::from(v))
            }
        }

        impl<FX, A, B> From<(A, B)> for $t
        where
            FX: Effect,
            Wise<FX>: From<A> + From<B>,
        {
            fn from(v: (A, B)) -> Self {
                <$t>::from(<$s>::from(v))
            }
        }
    };
}

impl_entrypoints!(Wise<FX> => BoxWise<FX>);
impl_entrypoints!(Expr<FX> => Wise<FX>);
impl_entrypoints!(Confined<FX> => Expr<FX>);

// Terminal entrypoint impls:
impl<FX> From<i32> for Confined<FX>
where
    FX: Effect,
{
    fn from(i: i32) -> Self {
        Confined::from(PrimVal::from(Num::from(i)))
    }
}

impl<FX> From<&'static str> for Confined<FX>
where
    FX: Effect,
{
    fn from(s: &'static str) -> Self {
        Confined::from(RcId::from(s))
    }
}

impl<FX> From<()> for Confined<FX>
where
    FX: Effect,
{
    fn from(_: ()) -> Self {
        Self::from(ListForm::default())
    }
}

impl<FX, T, const K: usize> From<[T; K]> for Confined<FX>
where
    FX: Effect,
    Wise<FX>: From<T>,
{
    fn from(v: [T; K]) -> Self {
        Confined::ListExpr(v.into_iter().map(Wise::from).map(Left).collect())
    }
}

impl<FX, A, B> From<(A, B)> for Confined<FX>
where
    FX: Effect,
    Wise<FX>: From<A> + From<B>,
{
    fn from((a, b): (A, B)) -> Self {
        Self::from([Wise::from(a), Wise::from(b)])
    }
}

// Bigger expr entrypoints:
impl<FX> From<Let<FX>> for Wise<FX>
where
    FX: Effect,
{
    fn from(letx: Let<FX>) -> Self {
        Wise::from(Expr::from(letx))
    }
}

// intermediate plumbing / non-entrypoints:
impl<FX> From<Expr<FX>> for Wise<FX>
where
    FX: Effect,
{
    fn from(x: Expr<FX>) -> Self {
        let fake_code = "<TEST FAKE CODE>";
        Self(WithSource::new(
            x,
            SourceCodeRef::new(
                Source::Literal(fake_code.to_string()).load().unwrap(),
                0..fake_code.len(),
            ),
        ))
    }
}

impl<FX> From<Confined<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(c: Confined<FX>) -> Self {
        Expr::from(Applications::from(LeftAssoc::just_left(Lookups::from(
            LeftAssoc::just_left(Interactions::new(vec![], c)),
        ))))
    }
}

impl<L, R> LeftAssoc<L, R> {
    fn just_left(left: L) -> Self {
        LeftAssoc::new(left, vec![])
    }
}
