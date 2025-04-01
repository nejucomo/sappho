mod clause;

use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_keyword::Keyword::Match as KwMatch;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider, ProcWiseParser};

pub use self::clause::MatchClause;

#[derive(Clone, Debug, PartialEq, new)]
pub struct Match<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    #[new(into)]
    pub candidate: BoxWise<K, FX>,
    pub clauses: Vec<MatchClause<K, FX>>,
}

impl<K, FX> ParsableWith<ProcWiseParser<'_, K>> for Match<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
    FX: Effect,
{
    fn make_parser_with(sep: ProcWiseParser<'_, K>) -> impl Parser<Self> {
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

impl<K, FX> Unparse for Match<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse,
    FX: Effect,
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

impl<K, FX> RestrictFrom<Match<K, ProcEffect>> for Match<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
    FX: Effect,
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
