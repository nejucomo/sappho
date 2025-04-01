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
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>> + Unparse,
    K::Expr<ProcEffect>: ParsableWith<ProcWiseParser<'a, K>>,
    FX: Effect,
{
    fn make_parser_with(sclink: Option<&'a SourceCodeLink>) -> impl Parser<Self> {
        chumsky::recursive::recursive(|proc| {
            WithSource::parser_with((sclink, proc)).map(Wise::from)
        })
        .restricted()
    }
}

/// This impl terminates recursion within the parser layer
impl<K, FX> ParsableWith<ProcWiseParser<'_, K>> for Wise<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>> + Unparse,
    FX: Effect,
{
    fn make_parser_with(proc: ProcWiseParser<'_, K>) -> impl Parser<Self> {
        proc.restricted()
    }
}

impl<K, FX> Unparse for Wise<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<K, FX> RestrictFrom<Wise<K, ProcEffect>> for Wise<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
    FX: Effect,
{
    fn restrict(src: Wise<K, ProcEffect>) -> Result<Wise<K, FX>, Restriction> {
        WithSource::restrict(src.0).map(Wise::from)
    }
}
