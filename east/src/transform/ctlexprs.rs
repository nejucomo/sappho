use sappho_effect::Effect;
use sappho_syntax as syntax;

use crate::transform::TransformInto;
use crate::{Let, LetClause, Match, MatchClause};

impl<FX> TransformInto<Let<FX>> for syntax::Let<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Let<FX> {
        Let::new(self.clauses.transform_into(), self.inner.transform_into())
    }
}

impl<FX> TransformInto<LetClause<FX>> for syntax::LetClause<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> LetClause<FX> {
        LetClause::new(self.binding, self.definition.transform_into())
    }
}

impl<FX> TransformInto<Match<FX>> for syntax::Match<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Match<FX> {
        Match::new(
            self.candidate.transform_into(),
            self.clauses.transform_into(),
        )
    }
}

impl<FX> TransformInto<MatchClause<FX>> for syntax::MatchClause<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> MatchClause<FX> {
        MatchClause::new(self.binding, self.consequent.transform_into())
    }
}
