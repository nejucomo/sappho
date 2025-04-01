use chumsky::Parser as _;
use derive_more::From;
use derive_new::new;
use sappho_effect::{ProcEffect, RestrictFrom};
use sappho_keyword::Keyword::Proc as KwProc;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider, ProcWiseParser};

#[derive(Clone, Debug, PartialEq, From, new)]
pub struct ProcDef<K>(#[new(into)] BoxWise<K, ProcEffect>)
where
    K: KastProvider;

impl<K> ProcDef<K>
where
    K: KastProvider,
{
    pub fn unwrap(self) -> BoxWise<K, ProcEffect> {
        self.0
    }
}

impl<K> ParsableWith<ProcWiseParser<'_, K>> for ProcDef<K>
where
    K: KastProvider,
    K::Expr<ProcEffect>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
{
    fn make_parser_with(sep: ProcWiseParser<'_, K>) -> impl Parser<Self> {
        KwProc
            .parse()
            .then_space()
            .ignore_then(bracketed(['{', '}'], BoxWise::parser_with(sep)))
            .map(Self)
    }
}

impl<K> Unparse for ProcDef<K>
where
    K: KastProvider,
    K::Expr<ProcEffect>: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Brackets::Squiggle;
        use sappho_unparse::Break;

        s.write(&KwProc);
        s.write(" ");
        s.bracketed(Squiggle, |subs| {
            subs.write(&Break::Mandatory);
            subs.write(&self.0);
        });
    }
}
