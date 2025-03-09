use chumsky::Parser as _;
use sappho_primval::PrimVal;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

/// Universal Nonrecursive eXpressions
#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum Unx {
    #[from(i32)]
    PrimVal(PrimVal),
    #[from]
    Deref(ArcId),
}

impl Parsable for Unx {
    fn parser() -> impl Parser<Self> {
        PrimVal::parser()
            .map(Unx::PrimVal)
            .or(ArcId::parser().map(Unx::Deref))
    }
}

impl Unparse for Unx {
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Unx::PrimVal(x) => x.unparse_into(s),
            Unx::Deref(x) => x.unparse_into(s),
        }
    }
}

// Conversions

#[cfg(test)]
mod test_conversions {
    use either::Either::{self, Left, Right};
    use sappho_primval::PrimVal;
    use sappho_syntax_idstore::ArcId;
    use sappho_try_transform::{TryTransformFrom, TryTransformInto};

    use crate::Unx;

    impl TryTransformInto<PrimVal> for Unx {
        fn try_transform_into(self) -> Either<PrimVal, Self> {
            match self {
                Unx::PrimVal(x) => Left(x),
                other => Right(other),
            }
        }
    }

    impl TryTransformInto<i32> for Unx {
        fn try_transform_into(self) -> Either<i32, Self> {
            i32::try_transform_from_via::<PrimVal>(self)
        }
    }

    impl TryTransformInto<ArcId> for Unx {
        fn try_transform_into(self) -> Either<ArcId, Self> {
            match self {
                Unx::Deref(x) => Left(x),
                other => Right(other),
            }
        }
    }
}
