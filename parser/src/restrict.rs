use crate::error::BareError;
use crate::error::Span;
use sappho_ast::{Ast, Expr};
use sappho_ast_core::{
    ApplicationExpr, CoreExpr, Effect, EffectExpr, LetClause, LetExpr, LookupExpr, MatchClause,
    MatchExpr,
};
use sappho_ast_core::{ProcEffect, PureEffect, QueryEffect};

pub(crate) trait Restrict<S>: Sized {
    fn restrict(src: S, span: Span) -> Result<Self, BareError>;
}

impl Restrict<ProcEffect> for PureEffect {
    fn restrict(src: ProcEffect, span: Span) -> Result<Self, BareError> {
        use ProcEffect::*;

        Err(BareError::custom(
            span,
            format!(
                "pure expressions cannot contain {}",
                match src {
                    Inquire => "query effects, e.g. `$…`",
                    Invoke => "evoke effects, e.g. `!…`",
                }
            ),
        ))
    }
}

impl Restrict<ProcEffect> for QueryEffect {
    fn restrict(src: ProcEffect, span: Span) -> Result<Self, BareError> {
        match src {
            ProcEffect::Inquire => Ok(QueryEffect::Inquire),
            ProcEffect::Invoke => Err(BareError::custom(
                span,
                "query expressions cannot contain evoke effects, e.g. `!…`".to_string(),
            )),
        }
    }
}

impl<FXS, FXD> Restrict<Expr<FXS>> for Expr<FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: Expr<FXS>, span: Span) -> Result<Self, BareError> {
        use Expr::*;

        match src {
            Core(x) => CoreExpr::restrict(x, span).map(Core),
            Func(x) => Ok(Func(x)),
            Query(x) => Ok(Query(x)),
            Proc(x) => Ok(Proc(x)),
            List(x) => Ok(List(
                x.into_iter()
                    .map(|ei| {
                        ei.map_right(|bx| *bx)
                            .map(|expr| Expr::<FXD>::restrict(expr, span.clone()))
                            .map_right(|res| res.map(Box::new))
                            .factor_err()
                    })
                    .collect::<Result<_, _>>()?,
            )),
        }
    }
}

impl<FXS, FXD> Restrict<CoreExpr<Ast, FXS>> for CoreExpr<Ast, FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: CoreExpr<Ast, FXS>, span: Span) -> Result<Self, BareError> {
        use sappho_ast_core::CoreExpr::*;

        match src {
            Lit(x) => Ok(Lit(x)),
            Ref(x) => Ok(Ref(x)),
            Object(x) => x
                .into_try_map_values(|expr| Expr::<FXD>::restrict(expr, span.clone()))
                .map(Object),
            Let(x) => LetExpr::restrict(x, span).map(Let),
            Match(x) => MatchExpr::restrict(x, span).map(Match),
            Application(x) => ApplicationExpr::restrict(x, span).map(Application),
            Lookup(x) => LookupExpr::restrict(x, span).map(Lookup),
            Effect(x) => EffectExpr::restrict(x, span).map(Effect),
        }
    }
}

impl<FXS, FXD> Restrict<LetExpr<Ast, FXS>> for LetExpr<Ast, FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: LetExpr<Ast, FXS>, span: Span) -> Result<Self, BareError> {
        let clauses: Vec<LetClause<Ast, FXD>> = src
            .clauses
            .into_iter()
            .map(|lc| LetClause::<Ast, FXD>::restrict(lc, span.clone()))
            .collect::<Result<_, BareError>>()?;
        let tail = Box::new(Expr::<FXD>::restrict(*src.tail, span)?);

        Ok(LetExpr { clauses, tail })
    }
}

impl<FXS, FXD> Restrict<LetClause<Ast, FXS>> for LetClause<Ast, FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: LetClause<Ast, FXS>, span: Span) -> Result<Self, BareError> {
        Ok(LetClause {
            binding: src.binding,
            bindexpr: Box::new(Expr::<FXD>::restrict(*src.bindexpr, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<MatchExpr<Ast, FXS>> for MatchExpr<Ast, FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: MatchExpr<Ast, FXS>, span: Span) -> Result<Self, BareError> {
        Ok(MatchExpr {
            target: Box::new(Expr::<FXD>::restrict(*src.target, span.clone())?),
            clauses: src
                .clauses
                .into_iter()
                .map(|c| MatchClause::restrict(c, span.clone()))
                .collect::<Result<Vec<_>, BareError>>()?,
        })
    }
}

impl<FXS, FXD> Restrict<MatchClause<Ast, FXS>> for MatchClause<Ast, FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: MatchClause<Ast, FXS>, span: Span) -> Result<Self, BareError> {
        Ok(MatchClause {
            pattern: src.pattern,
            body: Box::new(Expr::<FXD>::restrict(*src.body, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<ApplicationExpr<Ast, FXS>> for ApplicationExpr<Ast, FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: ApplicationExpr<Ast, FXS>, span: Span) -> Result<Self, BareError> {
        Ok(ApplicationExpr {
            target: Box::new(Expr::<FXD>::restrict(*src.target, span.clone())?),
            argument: Box::new(Expr::<FXD>::restrict(*src.argument, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<LookupExpr<Ast, FXS>> for LookupExpr<Ast, FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: LookupExpr<Ast, FXS>, span: Span) -> Result<Self, BareError> {
        Ok(LookupExpr {
            target: Box::new(Expr::<FXD>::restrict(*src.target, span)?),
            attr: src.attr,
        })
    }
}

impl<FXS, FXD> Restrict<EffectExpr<Ast, FXS>> for EffectExpr<Ast, FXD>
where
    FXD: Effect + Restrict<FXS>,
    FXS: Effect,
{
    fn restrict(src: EffectExpr<Ast, FXS>, span: Span) -> Result<Self, BareError> {
        Ok(EffectExpr {
            effect: FXD::restrict(src.effect, span.clone())?,
            expr: Box::new(Expr::<FXD>::restrict(*src.expr, span)?),
        })
    }
}

impl<S, D> Restrict<Box<S>> for Box<D>
where
    D: Restrict<S>,
{
    fn restrict(src: Box<S>, span: Span) -> Result<Self, BareError> {
        let d = D::restrict(*src, span)?;
        Ok(Box::new(d))
    }
}
