use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_keyword::Keyword::Let as KwLet;
use sappho_parsable::{Parsable as _, ParsableWith, Parser};
use sappho_pattern::Pattern;
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider, ProcWiseParser};

#[derive(Clone, Debug, PartialEq)]
pub struct Let<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    clauses: Vec<LetClause<K, FX>>,
    inner: BoxWise<K, FX>,
}

#[derive(Clone, Debug, PartialEq, new)]
pub struct LetClause<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    #[new(into)]
    binding: Pattern,
    #[new(into)]
    definition: BoxWise<K, FX>,
}

impl<K, FX> Let<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    pub fn new<I, C, W>(clauses: I, inner: W) -> Self
    where
        I: IntoIterator<Item = C>,
        LetClause<K, FX>: From<C>,
        BoxWise<K, FX>: From<W>,
    {
        Let {
            clauses: clauses.into_iter().map(LetClause::from).collect(),
            inner: BoxWise::from(inner),
        }
    }
}

impl<K, FX, P, W> From<(P, W)> for LetClause<K, FX>
where
    K: KastProvider,
    FX: Effect,
    Pattern: From<P>,
    BoxWise<K, FX>: From<W>,
{
    fn from((p, w): (P, W)) -> Self {
        Self::new(p, w)
    }
}

impl<K, FX> ParsableWith<ProcWiseParser<'_, K>> for Let<K, FX>
where
    K: KastProvider,
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ProcWiseParser<'_, K>) -> impl Parser<Self> {
        LetClause::parser_with(pep.clone())
            .then_space()
            .repeated()
            .at_least(1)
            .then(BoxWise::parser_with(pep))
            .map(|(clauses, inner)| Let::new(clauses, inner))
            .labelled("let-expression")
    }
}

impl<K, FX> ParsableWith<ProcWiseParser<'_, K>> for LetClause<K, FX>
where
    K: KastProvider,
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ProcWiseParser<'_, K>) -> impl Parser<Self> {
        KwLet
            .parse()
            .ignore_then(Pattern::parser())
            .then_ignore(just('=').opt_space_around())
            .then(BoxWise::parser_with(pep))
            .then_ignore(just(';'))
            .map(|(binding, definition)| LetClause::new(binding, definition))
    }
}

impl<K, FX> Unparse for Let<K, FX>
where
    K: KastProvider,
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

impl<K, FX> Unparse for LetClause<K, FX>
where
    K: KastProvider,
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

impl<K, FX> RestrictFrom<Let<K, ProcEffect>> for Let<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn restrict(src: Let<K, ProcEffect>) -> Result<Let<K, FX>, Restriction> {
        let clauses = src
            .clauses
            .into_iter()
            .map(LetClause::restrict)
            .collect::<Result<Vec<_>, _>>()?;
        let inner = BoxWise::restrict(src.inner)?;

        Ok(Let { clauses, inner })
    }
}

impl<K, FX> RestrictFrom<LetClause<K, ProcEffect>> for LetClause<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn restrict(src: LetClause<K, ProcEffect>) -> Result<LetClause<K, FX>, Restriction> {
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
