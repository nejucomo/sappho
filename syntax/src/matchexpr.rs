use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_keyword::Keyword::Match as KwMatch;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{BoxWise, Match, MatchClause, Pattern};

impl ParsableWith<ParseParams<'_>> for Match<ProcEffect> {
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

impl ParsableWith<ParseParams<'_>> for MatchClause<ProcEffect> {
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
