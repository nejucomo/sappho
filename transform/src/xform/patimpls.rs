use either::Either::{self, Left, Right};
use sappho_ast_reduced::{self as red};
use sappho_ast_rich::{self as ast};
use sappho_syntax_idstore::ArcId;

use crate::xform::listimpls::TailOrAttrs;
use crate::xform::{TransformInto, TryTransformInto};

impl TransformInto<red::Pattern> for ast::Pattern {
    fn transform(self) -> red::Pattern {
        match self {
            ast::Pattern::Bind(x) => red::Pattern::Bind(x),
            ast::Pattern::LitEq(x) => red::Pattern::LitEq(x),
            ast::Pattern::Unpack(x) => red::Pattern::Unpack(x.transform()),
            ast::Pattern::List(x) => x.transform(),
        }
    }
}

impl TransformInto<ast::Pattern> for red::Pattern {
    fn transform(self) -> ast::Pattern {
        match self {
            red::Pattern::Bind(x) => ast::Pattern::Bind(x),
            red::Pattern::LitEq(x) => ast::Pattern::LitEq(x),
            red::Pattern::Unpack(attrs) => {
                attrs.try_transform().either(ast::Pattern::List, |attrs| {
                    ast::Pattern::Unpack(attrs.transform())
                })
            }
        }
    }
}

impl TryTransformInto<TailOrAttrs<ArcId, red::Pattern>> for red::Pattern {
    fn try_transform(self) -> Either<TailOrAttrs<ArcId, red::Pattern>, Self> {
        use TailOrAttrs::*;

        match self {
            red::Pattern::Bind(rcid) => Left(Tail(rcid)),
            red::Pattern::Unpack(attrs) => Left(TailAttrs(attrs)),
            other => Right(other),
        }
    }
}
