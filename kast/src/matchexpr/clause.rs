use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{Parsable as _, ParsableWith, Parser};
use sappho_pattern::Pattern;
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider, ProcWiseParser};

#[derive(Clone, Debug, PartialEq, new)]
pub struct MatchClause<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    binding: Pattern,
    #[new(into)]
    consequent: BoxWise<K, FX>,
}

impl<K, FX> ParsableWith<ProcWiseParser<'_, K>> for MatchClause<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
    FX: Effect,
{
    fn make_parser_with(sep: ProcWiseParser<'_, K>) -> impl Parser<Self> {
        Pattern::parser()
            .then_ignore(just("->").space_around())
            .then(BoxWise::parser_with(sep))
            .map(|(binding, consequent)| Self::new(binding, consequent))
    }
}

impl<K, FX> Unparse for MatchClause<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.consequent);
    }
}

impl<K, FX> RestrictFrom<MatchClause<K, ProcEffect>> for MatchClause<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
    FX: Effect,
{
    fn restrict(src: MatchClause<K, ProcEffect>) -> Result<MatchClause<K, FX>, Restriction> {
        let MatchClause {
            binding,
            consequent,
        } = src;

        let consequent = BoxWise::restrict(consequent)?;

        Ok(MatchClause {
            binding,
            consequent,
        })
    }
}
