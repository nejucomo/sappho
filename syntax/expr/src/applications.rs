use chumsky::Parser as _;
use sappho_syntax_parsable::{Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Expr, InnerExpr, Lookups};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Applications {
    lookups: Lookups,
    args: Vec<InnerExpr>,
}

impl RecursiveParsable<Expr> for Applications {
    fn recursive_parser(rec: Recursive<'_, Expr>) -> impl Parser<Self> {
        Lookups::recursive_parser(rec.clone())
            .then(InnerExpr::recursive_parser(rec).repeated())
            .map(Applications::from)
    }
}

impl Unparse for Applications {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.lookups);
        for arg in &self.args {
            s.write(" ");
            s.write(arg);
        }
    }
}

// Conversions
#[cfg(test)]
mod test_conversions {
    use sappho_try_transform::TryTransformFrom;

    use crate::{Applications, Lookups};

    impl From<Lookups> for Applications {
        fn from(v: Lookups) -> Self {
            Self::from((v, vec![]))
        }
    }

    impl From<i32> for Applications {
        fn from(value: i32) -> Self {
            Self::from(Lookups::from(value))
        }
    }

    impl TryTransformFrom<Applications> for i32 {
        fn try_transform_from(src: Applications) -> either::Either<Self, Applications> {
            Self::try_transform_from(src.lookups).map_right(Applications::from)
        }
    }
}
