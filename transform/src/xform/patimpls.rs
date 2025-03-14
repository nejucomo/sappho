use either::Either::{self, Left, Right};
use sappho_ast_red as red;
use sappho_ast_rich as rich;
use sappho_identifier::RcId;

use crate::xform::listimpls::TailOrAttrs;
use crate::xform::{TransformInto, TryTransformInto};

impl TransformInto<red::Pattern> for rich::Pattern {
    fn transform(self) -> red::Pattern {
        match self {
            rich::Pattern::Bind(x) => red::Pattern::Bind(x),
            rich::Pattern::LitEq(x) => red::Pattern::LitEq(x),
            rich::Pattern::Unpack(x) => red::Pattern::Unpack(x.transform()),
            rich::Pattern::List(x) => x.transform(),
        }
    }
}

impl TransformInto<rich::Pattern> for red::Pattern {
    fn transform(self) -> rich::Pattern {
        match self {
            red::Pattern::Bind(x) => rich::Pattern::Bind(x),
            red::Pattern::LitEq(x) => rich::Pattern::LitEq(x),
            red::Pattern::Unpack(attrs) => {
                attrs.try_transform().either(rich::Pattern::List, |attrs| {
                    rich::Pattern::Unpack(attrs.transform())
                })
            }
        }
    }
}

impl TryTransformInto<TailOrAttrs<RcId, red::Pattern>> for red::Pattern {
    fn try_transform(self) -> Either<TailOrAttrs<RcId, red::Pattern>, Self> {
        use TailOrAttrs::*;

        match self {
            red::Pattern::Bind(rcid) => Left(Tail(rcid)),
            red::Pattern::Unpack(attrs) => Left(TailAttrs(attrs)),
            other => Right(other),
        }
    }
}
