use chumsky::Parser as _;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, RestrictableParser as _, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_source::SourceCodeLink;
use sappho_unparse::{Stream, Unparse};
use sappho_with_source::WithSource;

use crate::{KastProvider, ProcWiseParser, Wise};

impl<'a, K, FX> ParsableWith<Option<&'a SourceCodeLink>> for Wise<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<ProcEffect>: ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
    K::Expr<FX>: ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
{
    fn make_parser_with(sclink: Option<&'a SourceCodeLink>) -> impl Parser<Self> {
        chumsky::recursive::recursive(|proc| {
            WithSource::parser_with((sclink, proc)).map(Wise::from)
        })
        .restricted()
    }
}

/// This impl terminates recursion within the parser layer
impl<'a, K, FX> ParsableWith<ProcWiseParser<'a, K>> for Wise<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
{
    fn make_parser_with(proc: ProcWiseParser<'a, K>) -> impl Parser<Self> {
        proc.restricted()
    }
}

impl<K, FX> Unparse for Wise<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<K, FX> RestrictFrom<Wise<K, ProcEffect>> for Wise<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
{
    fn restrict(src: Wise<K, ProcEffect>) -> Result<Wise<K, FX>, Restriction> {
        WithSource::restrict(src.0).map(Wise::from)
    }
}
