use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_keyword::Keyword::Let as KwLet;
use sappho_parsable::{Parsable as _, ParsableWith, Parser};
use sappho_pattern::Pattern;
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider, ProcWiseParser};

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

impl<K, FX> ParsableWith<ProcWiseParser<'_, K>> for LetClause<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
    FX: Effect,
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

impl<K, FX> Unparse for LetClause<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: Unparse,
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

impl<K, FX> RestrictFrom<LetClause<K, ProcEffect>> for LetClause<K, FX>
where
    K: KastProvider,
    K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
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
