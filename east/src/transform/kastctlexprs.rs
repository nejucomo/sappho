use crate::transform::{TransformInto, TransformWithSource};
use sappho_effect::Effect;
use sappho_kast::{KastProvider, Let, LetClause, Match, MatchClause};

impl<KS, KT, FX> TransformInto<Let<KT, FX>> for Let<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformWithSource<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> Let<KT, FX> {
        Let::new(self.clauses.transform_into(), self.inner.transform_into())
    }
}

impl<KS, KT, FX> TransformInto<LetClause<KT, FX>> for LetClause<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformWithSource<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> LetClause<KT, FX> {
        LetClause::new(self.binding, self.definition.transform_into())
    }
}

impl<KS, KT, FX> TransformInto<Match<KT, FX>> for Match<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformWithSource<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> Match<KT, FX> {
        Match::new(
            self.candidate.transform_into(),
            self.clauses.transform_into(),
        )
    }
}

impl<KS, KT, FX> TransformInto<MatchClause<KT, FX>> for MatchClause<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformWithSource<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> MatchClause<KT, FX> {
        MatchClause::new(self.binding, self.consequent.transform_into())
    }
}
