use either::Either::{self, Left, Right};
use sappho_ast_red::{self as astred};
use sappho_ast_rich as rich;
use sappho_identifier::RcId;

use crate::xform::listimpls::TailOrAttrs;
use crate::xform::{TransformInto, TryTransformInto};

impl TransformInto<astred::Pattern> for rich::Pattern {
    fn transform(self) -> astred::Pattern {
        match self {
            rich::Pattern::Bind(x) => astred::Pattern::Bind(x),
            rich::Pattern::LitEq(x) => astred::Pattern::LitEq(x),
            rich::Pattern::Unpack(x) => astred::Pattern::Unpack(x.transform()),
            rich::Pattern::List(x) => x.transform(),
        }
    }
}

impl TransformInto<rich::Pattern> for astred::Pattern {
    fn transform(self) -> rich::Pattern {
        match self {
            astred::Pattern::Bind(x) => rich::Pattern::Bind(x),
            astred::Pattern::LitEq(x) => rich::Pattern::LitEq(x),
            astred::Pattern::Unpack(attrs) => {
                attrs.try_transform().either(rich::Pattern::List, |attrs| {
                    rich::Pattern::Unpack(attrs.transform())
                })
            }
        }
    }
}

impl TryTransformInto<TailOrAttrs<RcId, astred::Pattern>> for astred::Pattern {
    fn try_transform(self) -> Either<TailOrAttrs<RcId, astred::Pattern>, Self> {
        use TailOrAttrs::*;

        match self {
            astred::Pattern::Bind(rcid) => Left(Tail(rcid)),
            astred::Pattern::Unpack(attrs) => Left(TailAttrs(attrs)),
            other => Right(other),
        }
    }
}
