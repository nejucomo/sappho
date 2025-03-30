use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_keyword::Keyword::Match as KwMatch;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{BoxWise, Pattern};

#[derive(Debug, PartialEq, new)]
pub struct Match<FX>
where
    FX: Effect,
{
    #[new(into)]
    candidate: BoxWise<FX>,
    clauses: Vec<MatchClause<FX>>,
}

#[derive(Debug, PartialEq, new)]
pub struct MatchClause<FX>
where
    FX: Effect,
{
    binding: Pattern,
    #[new(into)]
    consequent: BoxWise<FX>,
}

impl<FX> ParsableWith<ParseParams<'_>> for Match<FX>
where
    FX: Effect,
{
    fn make_parser_with(sep: ParseParams<'_>) -> impl Parser<Self> {
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

impl<FX> ParsableWith<ParseParams<'_>> for MatchClause<FX>
where
    FX: Effect,
{
    fn make_parser_with(sep: ParseParams<'_>) -> impl Parser<Self> {
        Pattern::parser()
            .then_ignore(just("->").space_around())
            .then(BoxWise::parser_with(sep))
            .map(|(binding, consequent)| Self::new(binding, consequent))
    }
}

impl<FX> Unparse for Match<FX>
where
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

impl<FX> Unparse for MatchClause<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.consequent);
    }
}

impl<FX> RestrictFrom<Match<ProcEffect>> for Match<FX>
where
    FX: Effect,
{
    fn restrict(src: Match<ProcEffect>) -> Result<Match<FX>, Restriction> {
        let clauses = src
            .clauses
            .into_iter()
            .map(MatchClause::restrict)
            .collect::<Result<Vec<_>, _>>()?;
        let candidate = BoxWise::restrict(src.candidate)?;

        Ok(Match { candidate, clauses })
    }
}

impl<FX> RestrictFrom<MatchClause<ProcEffect>> for MatchClause<FX>
where
    FX: Effect,
{
    fn restrict(src: MatchClause<ProcEffect>) -> Result<MatchClause<FX>, Restriction> {
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
