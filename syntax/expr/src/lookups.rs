use chumsky::Parser as _;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_leftassoc::{from_via_leftassoc_left, LeftAssoc};
use sappho_syntax_parsable::{ParsableWith, Parser, Recursive};
use sappho_syntax_universal::Unx;
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{AttrLookup, Expr, InnerExpr, ListExpr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Lookups(LeftAssoc<InnerExpr, AttrLookup>);

from_via_leftassoc_left!(InnerExpr => Lookups);
from_via_leftassoc_left!(Unx => Lookups);
from_via_leftassoc_left!(i32 => Lookups);
from_via_leftassoc_left!(ArcId => Lookups);
from_via_leftassoc_left!(ListExpr => Lookups);
from_via_leftassoc_left!(Vec<Expr> => Lookups);

impl Lookups {
    pub fn la_fold<F, A>(self, f: F) -> A
    where
        A: From<InnerExpr>,
        F: Fn(A, AttrLookup) -> A,
    {
        self.0.la_fold(f)
    }
}

impl<'r> ParsableWith<Recursive<'r, Expr>> for Lookups {
    fn make_parser_with(expr: Recursive<'r, Expr>) -> impl Parser<Self> {
        LeftAssoc::make_parser_with((expr, ())).map(Lookups)
    }
}

impl Unparse for Lookups {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

#[cfg(test)]
impl sappho_try_transform::TryTransformInto<InnerExpr> for Lookups {
    fn try_transform_into(self) -> either::Either<InnerExpr, Self> {
        self.0.try_transform_into().map_right(Lookups)
    }
}
