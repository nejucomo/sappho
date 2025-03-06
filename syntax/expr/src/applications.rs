use chumsky::Parser as _;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{InnerExpr, Lookups};

#[derive(Debug, derive_more::From)]
pub struct Applications {
    base: Lookups,
    args: Vec<InnerExpr>,
}

impl Parsable for Applications {
    fn parser() -> impl Parser<Self> {
        Lookups::parser()
            .then(InnerExpr::parser().repeated())
            .map(Applications::from)
    }
}

impl Unparse for Applications {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.base);
        for arg in &self.args {
            s.write(" ");
            s.write(arg);
        }
    }
}
