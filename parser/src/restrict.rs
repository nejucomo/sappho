use crate::error::BareError;
use crate::error::Span;
use sappho_ast::{
    ApplicationExpr, CoreExpr, EffectExpr, Expr, LetClause, LetExpr, LookupExpr, MatchClause,
    MatchExpr,
};
use sappho_ast_core::{ProcEffects, PureEffects, QueryEffects};

pub(crate) trait Restrict<S>: Sized {
    fn restrict(src: S, span: Span) -> Result<Self, BareError>;
}

impl Restrict<ProcEffects> for PureEffects {
    fn restrict(src: ProcEffects, span: Span) -> Result<Self, BareError> {
        use ProcEffects::*;

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

impl Restrict<ProcEffects> for QueryEffects {
    fn restrict(src: ProcEffects, span: Span) -> Result<Self, BareError> {
        match src {
            ProcEffects::Inquire => Ok(QueryEffects::Inquire),
            ProcEffects::Invoke => Err(BareError::custom(
                span,
                "query expressions cannot contain evoke effects, e.g. `!…`".to_string(),
            )),
        }
    }
}

impl<FXS, FXD> Restrict<Expr<FXS>> for Expr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: Expr<FXS>, span: Span) -> Result<Self, BareError> {
        use Expr::*;

        match src {
            Core(x) => CoreExpr::restrict(x, span).map(Core),
            Func(x) => Ok(Func(x)),
            Query(x) => Ok(Query(x)),
            Proc(x) => Ok(Proc(x)),
            List(x) => {
                let tailspan = span.clone();
                Ok(List(x.try_map(
                    move |elem| Expr::<FXD>::restrict(elem, span.clone()),
                    move |tail| Expr::<FXD>::restrict(*tail, tailspan).map(Box::new),
                )?))
            }
        }
    }
}

impl<FXS, FXD> Restrict<CoreExpr<FXS>> for CoreExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: CoreExpr<FXS>, span: Span) -> Result<Self, BareError> {
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

impl<FXS, FXD> Restrict<LetExpr<FXS>> for LetExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: LetExpr<FXS>, span: Span) -> Result<Self, BareError> {
        let clauses: Vec<LetClause<FXD>> = src
            .clauses
            .into_iter()
            .map(|lc| LetClause::<FXD>::restrict(lc, span.clone()))
            .collect::<Result<_, BareError>>()?;
        let tail = Box::new(Expr::<FXD>::restrict(*src.tail, span)?);

        Ok(LetExpr { clauses, tail })
    }
}

impl<FXS, FXD> Restrict<LetClause<FXS>> for LetClause<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: LetClause<FXS>, span: Span) -> Result<Self, BareError> {
        Ok(LetClause {
            binding: src.binding,
            bindexpr: Box::new(Expr::<FXD>::restrict(*src.bindexpr, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<MatchExpr<FXS>> for MatchExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: MatchExpr<FXS>, span: Span) -> Result<Self, BareError> {
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

impl<FXS, FXD> Restrict<MatchClause<FXS>> for MatchClause<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: MatchClause<FXS>, span: Span) -> Result<Self, BareError> {
        Ok(MatchClause {
            pattern: src.pattern,
            body: Box::new(Expr::<FXD>::restrict(*src.body, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<ApplicationExpr<FXS>> for ApplicationExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: ApplicationExpr<FXS>, span: Span) -> Result<Self, BareError> {
        Ok(ApplicationExpr {
            target: Box::new(Expr::<FXD>::restrict(*src.target, span.clone())?),
            argument: Box::new(Expr::<FXD>::restrict(*src.argument, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<LookupExpr<FXS>> for LookupExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: LookupExpr<FXS>, span: Span) -> Result<Self, BareError> {
        Ok(LookupExpr {
            target: Box::new(Expr::<FXD>::restrict(*src.target, span)?),
            attr: src.attr,
        })
    }
}

impl<FXS, FXD> Restrict<EffectExpr<FXS>> for EffectExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: EffectExpr<FXS>, span: Span) -> Result<Self, BareError> {
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
