use sappho_ast_kernel::{Expression, Kernel, Pattern};
use sappho_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_more::From)]
pub struct RedExpr(Kernel<Self>);

#[derive(Debug, derive_more::From)]
pub struct RedPat(Pattern<RedExpr>);

impl Expression for RedExpr {
    type Pattern = RedPat;
}

impl Parsable for RedExpr {
    fn parser() -> impl Parser<Self> {
        chumsky::recursive::recursive(Self::parser_with)
    }
}

impl ParsableWith<Recursive<'_, RedExpr>> for RedExpr {
    fn make_parser_with(expr: Recursive<'_, RedExpr>) -> impl Parser<Self> {
        Kernel::parser_with(expr).map(RedExpr)
    }
}

impl Unparse for RedExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
