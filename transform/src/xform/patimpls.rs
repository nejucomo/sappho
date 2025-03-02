use either::Either::{self, Left, Right};
use sappho_ast::{self as ast};
use sappho_ast_reduced::{self as astred};
use sappho_attrs::Attrs;
use sappho_identifier::RcId;

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

impl TryTransformInto<Either<RcId, Attrs<astred::Pattern>>> for astred::Pattern {
    fn try_transform(self) -> Either<Either<RcId, Attrs<astred::Pattern>>, Self> {
        match self {
            astred::Pattern::Bind(rcid) => Left(Left(rcid)),
            astred::Pattern::Unpack(attrs) => Left(Right(attrs)),
            other => Right(other),
        }
    }
}
