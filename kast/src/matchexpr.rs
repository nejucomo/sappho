use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_keyword::Keyword::Match as KwMatch;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{Parsable as _, ParsableWith, Parser};
use sappho_pattern::Pattern;
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider, ProcWiseParser};

#[derive(Clone, Debug, PartialEq, new)]
pub struct Match<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    #[new(into)]
    candidate: BoxWise<K, FX>,
    clauses: Vec<MatchClause<K, FX>>,
}

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

impl<'a, K, FX> ParsableWith<ProcWiseParser<'a, K>> for Match<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
{
    fn make_parser_with(sep: ProcWiseParser<'a, K>) -> impl Parser<Self> {
        KwMatch
            .parse()
            .then_space()
            .ignore_then(BoxWise::parser_with(sep.clone()))
            .then_space()
            .then(bracketed(
                ['{', '}'],
                MatchClause::parser_with(sep)
                    .separated_by(just(',').then_opt_space())
                    .allow_trailing(),
            ))
            .map(|(candidate, clauses)| Match::new(candidate, clauses))
    }
}

impl<'a, K, FX> ParsableWith<ProcWiseParser<'a, K>> for MatchClause<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
{
    fn make_parser_with(sep: ProcWiseParser<'a, K>) -> impl Parser<Self> {
        Pattern::parser()
            .then_ignore(just("->").space_around())
            .then(BoxWise::parser_with(sep))
            .map(|(binding, consequent)| Self::new(binding, consequent))
    }
}

impl<K, FX> Unparse for Match<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Brackets::Squiggle, Break::OptSpace};

        s.write(&KwMatch);
        s.write(" ");
        s.write(&self.candidate);
        s.write(" ");
        s.bracketed(Squiggle, |subs| {
            for clause in &self.clauses {
                subs.write(&OptSpace);
                subs.write(clause);
                subs.write(",");
            }
        });
    }
}

impl<K, FX> Unparse for MatchClause<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.consequent);
    }
}

impl<K, FX> RestrictFrom<Match<K, ProcEffect>> for Match<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
{
    fn restrict(src: Match<K, ProcEffect>) -> Result<Match<K, FX>, Restriction> {
        let clauses = src
            .clauses
            .into_iter()
            .map(MatchClause::restrict)
            .collect::<Result<Vec<_>, _>>()?;
        let candidate = BoxWise::restrict(src.candidate)?;

        Ok(Match { candidate, clauses })
    }
}

impl<K, FX> RestrictFrom<MatchClause<K, ProcEffect>> for MatchClause<K, FX>
where
    K: KastProvider,
    FX: Effect,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
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
