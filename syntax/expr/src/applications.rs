use chumsky::Parser as _;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_leftassoc::{from_via_leftassoc_left, LeftAssoc};
use sappho_syntax_parsable::{ParsableWith, Parser, Recursive};
use sappho_syntax_universal::Unx;
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Expr, InnerExpr, ListExpr, Lookups};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Applications(LeftAssoc<Lookups, InnerExpr>);

from_via_leftassoc_left!(Lookups => Applications);
from_via_leftassoc_left!(InnerExpr => Applications);
from_via_leftassoc_left!(Unx => Applications);
from_via_leftassoc_left!(i32 => Applications);
from_via_leftassoc_left!(ArcId => Applications);
from_via_leftassoc_left!(ListExpr => Applications);
from_via_leftassoc_left!(Vec<Expr> => Applications);

impl Applications {
    pub fn la_fold<F, A>(self, f: F) -> A
    where
        A: From<Lookups>,
        F: Fn(A, InnerExpr) -> A,
    {
        self.0.la_fold(f)
    }
}

impl<'r> ParsableWith<Recursive<'r, Expr>> for Applications {
    fn make_parser_with(expr: Recursive<'r, Expr>) -> impl Parser<Self> {
        LeftAssoc::make_parser_with((expr.clone(), expr)).map(Applications)
    }
}

impl Unparse for Applications {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

#[cfg(test)]
impl sappho_try_transform::TryTransformInto<InnerExpr> for Applications {
    fn try_transform_into(self) -> either::Either<InnerExpr, Self> {
        use sappho_try_transform::TryTransformFrom;

        InnerExpr::try_transform_from_via::<Lookups>(self)
    }
}

#[cfg(test)]
impl sappho_try_transform::TryTransformInto<Lookups> for Applications {
    fn try_transform_into(self) -> either::Either<Lookups, Self> {
        self.0.try_transform_into().map_right(Applications)
    }
}
