use sappho_ast_core::{
    ApplicationExpr, AstProvider, EffectExpr, LetClause, LetExpr, LookupExpr, MatchClause,
    MatchExpr,
};
use sappho_ast_effect::Effect;

use crate::xform::TransformInto;

impl<XPS, XPD, FX> TransformInto<LetExpr<XPD, FX>> for LetExpr<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
    XPS::Pattern: TransformInto<XPD::Pattern>,
{
    fn transform(self) -> LetExpr<XPD, FX> {
        LetExpr::new(
            self.clauses.into_iter().map(LetClause::transform).collect(),
            Box::new(self.tail.transform()),
        )
    }
}

impl<XPS, XPD, FX> TransformInto<LetClause<XPD, FX>> for LetClause<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
    XPS::Pattern: TransformInto<XPD::Pattern>,
{
    fn transform(self) -> LetClause<XPD, FX> {
        LetClause::new(
            self.binding.transform(),
            Box::new(self.bindexpr.transform()),
        )
    }
}

impl<XPS, XPD, FX> TransformInto<MatchExpr<XPD, FX>> for MatchExpr<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
    XPS::Pattern: TransformInto<XPD::Pattern>,
{
    fn transform(self) -> MatchExpr<XPD, FX> {
        MatchExpr::new(
            Box::new(self.target.transform()),
            self.clauses
                .into_iter()
                .map(MatchClause::transform)
                .collect(),
        )
    }
}

impl<XPS, XPD, FX> TransformInto<MatchClause<XPD, FX>> for MatchClause<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
    XPS::Pattern: TransformInto<XPD::Pattern>,
{
    fn transform(self) -> MatchClause<XPD, FX> {
        MatchClause::new(self.pattern.transform(), Box::new(self.body.transform()))
    }
}

impl<XPS, XPD, FX> TransformInto<ApplicationExpr<XPD, FX>> for ApplicationExpr<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
{
    fn transform(self) -> ApplicationExpr<XPD, FX> {
        ApplicationExpr::new(
            Box::new(self.target.transform()),
            Box::new(self.argument.transform()),
        )
    }
}

impl<XPS, XPD, FX> TransformInto<LookupExpr<XPD, FX>> for LookupExpr<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
{
    fn transform(self) -> LookupExpr<XPD, FX> {
        LookupExpr::new(Box::new(self.target.transform()), self.attr)
    }
}

impl<XPS, XPD, FX> TransformInto<EffectExpr<XPD, FX>> for EffectExpr<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
{
    fn transform(self) -> EffectExpr<XPD, FX> {
        EffectExpr::new(self.effect, Box::new(self.expr.transform()))
    }
}
