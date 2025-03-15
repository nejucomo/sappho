use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_keyword::Keyword::Let as KwLet;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{BoxWise, Let, LetClause, Pattern};

impl<FX> ParsableWith<ParseParams<'_>> for Let<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        LetClause::parser_with(pep.clone())
            .then_space()
            .repeated()
            .at_least(1)
            .then(BoxWise::parser_with(pep))
            .map(|(clauses, inner)| Let::new(clauses, inner))
            .labelled("let-expression")
    }
}

impl<FX> ParsableWith<ParseParams<'_>> for LetClause<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        KwLet
            .parse()
            .ignore_then(Pattern::parser())
            .then_ignore(just('=').opt_space_around())
            .then(BoxWise::parser_with(pep))
            .then_ignore(just(';'))
            .map(|(binding, definition)| LetClause::new(binding, definition))
    }
}

impl<FX> Unparse for Let<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Brackets::Parens, Break::Mandatory};

        let unparse_clauses = |s: &mut Stream| {
            for (ix, clause) in self.clauses.iter().enumerate() {
                if s.depth() > 0 || ix > 0 {
                    s.write(&Mandatory);
                }
                s.write(clause);
                s.write(";");
            }
            s.write(&Mandatory);
            s.write(&self.inner);
        };

        if s.depth() == 0 {
            unparse_clauses(s);
        } else {
            s.bracketed(Parens, unparse_clauses);
        }
    }
}

impl<FX> Unparse for LetClause<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&KwLet);
        s.write(" ");
        s.write(&self.binding);
        s.write(" = ");
        s.write(&self.definition);
    }
}

impl<FX> RestrictFrom<Let<ProcEffect>> for Let<FX>
where
    FX: Effect,
{
    fn restrict(src: Let<ProcEffect>) -> Result<Let<FX>, Restriction> {
        let clauses = src
            .clauses
            .into_iter()
            .map(LetClause::restrict)
            .collect::<Result<Vec<_>, _>>()?;
        let inner = BoxWise::restrict(src.inner)?;

        Ok(Let { clauses, inner })
    }
}

impl<FX> RestrictFrom<LetClause<ProcEffect>> for LetClause<FX>
where
    FX: Effect,
{
    fn restrict(src: LetClause<ProcEffect>) -> Result<LetClause<FX>, Restriction> {
        let LetClause {
            binding,
            definition,
        } = src;

        let definition = BoxWise::restrict(definition)?;

        Ok(LetClause {
            binding,
            definition,
        })
    }
}
