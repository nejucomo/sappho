use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_keyword::Keyword::Let as KwLet;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{Let, LetClause, Pattern, SEParser, BSE};

impl ParsableWith<SEParser<'_>> for Let<ProcEffect> {
    fn make_parser_with(pep: SEParser<'_>) -> impl Parser<Self> {
        LetClause::parser_with(pep.clone())
            .then_space()
            .repeated()
            .at_least(1)
            .then(BSE::parser_with(pep))
            .map(|(clauses, inner)| Let::new(clauses, inner))
            .labelled("let-expression")
    }
}

impl ParsableWith<SEParser<'_>> for LetClause<ProcEffect> {
    fn make_parser_with(pep: SEParser<'_>) -> impl Parser<Self> {
        KwLet
            .parse()
            .ignore_then(Pattern::parser())
            .then_ignore(just('=').opt_space_around())
            .then(BSE::parser_with(pep))
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
