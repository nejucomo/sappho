mod clause;

use chumsky::Parser as _;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider, ProcWiseParser};

pub use self::clause::LetClause;

#[derive(Clone, Debug, PartialEq)]
pub struct Let<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    pub clauses: Vec<LetClause<K, FX>>,
    pub inner: BoxWise<K, FX>,
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

impl<K, FX> ParsableWith<ProcWiseParser<'_, K>> for Let<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
    FX: Effect,
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

impl<K, FX> Unparse for Let<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse,
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

impl<K, FX> RestrictFrom<Let<K, ProcEffect>> for Let<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
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
