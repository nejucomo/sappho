use either::Either::{self, Left, Right};
use sappho_ast::{self as ast};
use sappho_ast_reduced::{self as astred};
use sappho_identifier::RcId;

use crate::xform::listimpls::TailOrAttrs;
use crate::xform::{TransformInto, TryTransformInto};

impl TransformInto<astred::Pattern> for ast::Pattern {
    fn transform(self) -> astred::Pattern {
        match self {
            ast::Pattern::Bind(x) => astred::Pattern::Bind(x),
            ast::Pattern::LitEq(x) => astred::Pattern::LitEq(x),
            ast::Pattern::Unpack(x) => astred::Pattern::Unpack(x.transform()),
            ast::Pattern::List(x) => x.transform(),
        }
    }
}

impl TransformInto<ast::Pattern> for astred::Pattern {
    fn transform(self) -> ast::Pattern {
        match self {
            astred::Pattern::Bind(x) => ast::Pattern::Bind(x),
            astred::Pattern::LitEq(x) => ast::Pattern::LitEq(x),
            astred::Pattern::Unpack(attrs) => {
                attrs.try_transform().either(ast::Pattern::List, |attrs| {
                    ast::Pattern::Unpack(attrs.transform())
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
