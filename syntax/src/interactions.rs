use chumsky::Parser as _;
use derive_more::From;
use derive_new::new;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::Confined;

/// Potentially effectful expressions
#[derive(Debug, PartialEq, From, new)]
pub struct Interactions<FX>
where
    FX: Effect,
{
    #[new(into)]
    pub effects: Vec<FX>,
    #[new(into)]
    pub confined: Confined<FX>,
}

impl<FX, T> From<T> for Interactions<FX>
where
    FX: Effect,
    Confined<FX>: From<T>,
{
    fn from(value: T) -> Self {
        Self::new(vec![], Confined::from(value))
    }
}

impl<FX> ParsableWith<ParseParams<'_>> for Interactions<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        FX::parser()
            .repeated()
            .then(Confined::parser_with(pep))
            .map(Interactions::from)
    }
}

impl<FX> Unparse for Interactions<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        for fx in &self.effects {
            s.write(fx);
        }
        s.write(&self.confined);
    }
}

impl<FX> RestrictFrom<Interactions<ProcEffect>> for Interactions<FX>
where
    FX: Effect,
{
    fn restrict(src: Interactions<ProcEffect>) -> Result<Interactions<FX>, Restriction> {
        let effects = src
            .effects
            .into_iter()
            .map(|fx| FX::restrict(fx))
            .collect::<Result<Vec<_>, _>>()?;
        let confined = Confined::restrict(src.confined)?;
        Ok(Interactions { effects, confined })
    }
}
