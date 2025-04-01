use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;
use sappho_effect::{ProcEffect, PureEffect, RestrictFrom};
use sappho_keyword::Keyword::Fn;
use sappho_parsable::primitive::space;
use sappho_parsable::{Parsable as _, ParsableWith, Parser};
use sappho_pattern::Pattern;
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider, ProcWiseParser};

#[derive(Clone, Debug, PartialEq, new)]
pub struct FuncDef<K>
where
    K: KastProvider,
{
    #[new(into)]
    argpat: Pattern,
    #[new(into)]
    body: BoxWise<K, PureEffect>,
}

impl<K> ParsableWith<ProcWiseParser<'_, K>> for FuncDef<K>
where
    K: KastProvider,
    K::Expr<PureEffect>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
{
    fn make_parser_with(proc: ProcWiseParser<'_, K>) -> impl Parser<Self> {
        Fn.parse()
            .then_space()
            .ignore_then(Pattern::parser())
            .then_ignore(just("->").delimited_by(space(), space()))
            .then(BoxWise::parser_with(proc))
            .map(|(argpat, body)| FuncDef::new(argpat, body))
            .labelled("fn definition")
    }
}

impl<K> Unparse for FuncDef<K>
where
    K: KastProvider,
    K::Expr<PureEffect>: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&Fn);
        s.write(" ");
        s.write(&self.argpat);
        s.write(" -> ");
        s.write(&self.body);
    }
}
