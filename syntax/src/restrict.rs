use sappho_ast_effect::{Effect, EffectDescription};
use sappho_object::{Element, Object};
use sappho_parsable::error::{ChumskyError, Span};

use crate::leftassoc::LeftAssoc;
use crate::spanned::Spanned;
use crate::{
    Applications, Confined, EffectExpr, Expr, FuncDef, Let, LetClause, Lookup, Lookups, Match,
    MatchClause, ParensExpr, ProcDef, ProcExpr, PureExpr, QueryDef, QueryExpr,
};

pub(crate) trait RestrictInto<D> {
    fn restrict(self, span: Span) -> Result<D, ChumskyError>;
}

/// Location tracking in restriction
impl<FXS, FXD> RestrictInto<Spanned<FXS>> for Spanned<FXD>
where
    FXD: RestrictInto<FXS>,
{
    fn restrict(self, _: Span) -> Result<Spanned<FXS>, ChumskyError> {
        // We shadow the outer span with the new source span:
        self.node
            .restrict(self.span.clone())
            .map(|fxd| Spanned::new(fxd, self.span))
    }
}

// Restricting top-level exprs:
impl RestrictInto<PureExpr> for QueryExpr {
    fn restrict(self, span: Span) -> Result<PureExpr, ChumskyError> {
        self.0.restrict(span).map(PureExpr)
    }
}

impl RestrictInto<PureExpr> for ProcExpr {
    fn restrict(self, span: Span) -> Result<PureExpr, ChumskyError> {
        self.0.restrict(span).map(PureExpr)
    }
}

impl RestrictInto<QueryExpr> for ProcExpr {
    fn restrict(self, span: Span) -> Result<QueryExpr, ChumskyError> {
        self.0.restrict(span).map(QueryExpr)
    }
}

// Restricting effects exprs:
struct RestrictWrapper<FX>(FX);

impl<FXS, FXD> RestrictInto<FXD> for RestrictWrapper<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<FXD, ChumskyError> {
        FXD::try_from(self.0).map_err(|sfx| {
            let context = FXS::context();

            let EffectDescription { noun, sigil, .. } = sfx.description();
            ChumskyError::custom(
                span,
                format!("{noun} {sigil:?} restricted in {context} contexts"),
            )
        })
    }
}

// FX-generic restrictions:
impl<FXS, FXD> RestrictInto<Expr<FXD>> for Expr<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<Expr<FXD>, ChumskyError> {
        use Expr::*;

        match self {
            Func(x) => Ok(Func(x)),
            Query(x) => Ok(Query(x)),
            Proc(x) => Ok(Proc(x)),
            Let(x) => x.restrict(span).map(Let),
            Match(x) => x.restrict(span).map(Match),
            Applications(x) => x.restrict(span).map(Applications),
        }
    }
}

impl<FXS, FXD> RestrictInto<Let<FXD>> for Let<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<Let<FXD>, ChumskyError> {
        let clauses = self
            .clauses
            .into_iter()
            .map(|clause| clause.restrict(span.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let inner = self.inner.restrict(span)?;

        Ok(Let { clauses, inner })
    }
}

impl<FXS, FXD> RestrictInto<LetClause<FXD>> for LetClause<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<LetClause<FXD>, ChumskyError> {
        let LetClause {
            binding,
            definition,
        } = self;

        let definition = definition.restrict(span)?;

        Ok(LetClause {
            binding,
            definition,
        })
    }
}

impl<FXS, FXD> RestrictInto<Match<FXD>> for Match<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<Match<FXD>, ChumskyError> {
        let clauses = self
            .clauses
            .into_iter()
            .map(|clause| clause.restrict(span.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let candidate = self.candidate.restrict(span)?;

        Ok(Match { candidate, clauses })
    }
}

impl<FXS, FXD> RestrictInto<MatchClause<FXD>> for MatchClause<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<MatchClause<FXD>, ChumskyError> {
        let MatchClause {
            binding,
            consequent,
        } = self;

        let consequent = consequent.restrict(span)?;

        Ok(MatchClause {
            binding,
            consequent,
        })
    }
}

impl<FXS, FXD> RestrictInto<Applications<FXD>> for Applications<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<Applications<FXD>, ChumskyError> {
        self.0.restrict(span).map(Applications)
    }
}

impl<FXS, FXD> RestrictInto<Lookups<FXD>> for Lookups<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<Lookups<FXD>, ChumskyError> {
        self.0.restrict(span).map(Lookups)
    }
}

impl RestrictInto<Lookup> for Lookup {
    fn restrict(self, _: Span) -> Result<Lookup, ChumskyError> {
        Ok(self)
    }
}

impl<FXS, FXD> RestrictInto<EffectExpr<FXD>> for EffectExpr<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<EffectExpr<FXD>, ChumskyError> {
        let effects = self
            .effects
            .into_iter()
            .map(|fx| RestrictWrapper(fx).restrict(span.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let confined = self.confined.restrict(span)?;
        Ok(EffectExpr { effects, confined })
    }
}

impl<FXS, FXD> RestrictInto<Confined<FXD>> for Confined<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<Confined<FXD>, ChumskyError> {
        use Confined::*;

        match self {
            Ref(x) => Ok(Ref(x)),
            Prim(x) => Ok(Prim(x)),
            Parens(x) => x.restrict(span).map(Parens),
            ObjectDef(x) => x.restrict(span).map(ObjectDef),
            ListExpr(x) => x.restrict_into(span).map(ListExpr),
        }
    }
}

impl<FXS, FXD> RestrictInto<ParensExpr<FXD>> for ParensExpr<FXS>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(self, span: Span) -> Result<ParensExpr<FXD>, ChumskyError> {
        self.0.restrict(span).map(ParensExpr)
    }
}

impl<FXS, FXD> RestrictInto<Object<FuncDef, QueryDef, ProcDef, Expr<FXD>>>
    for Object<FuncDef, QueryDef, ProcDef, Expr<FXS>>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(
        self,
        span: Span,
    ) -> Result<Object<FuncDef, QueryDef, ProcDef, Expr<FXD>>, ChumskyError> {
        self.into_iter()
            .map(|x| x.restrict(span.clone()))
            .collect::<Result<Result<Object<_, _, _, _>, String>, ChumskyError>>()
            .map(|res: Result<_, String>| res.unwrap())
    }
}

impl<FXS, FXD> RestrictInto<Element<FuncDef, QueryDef, ProcDef, Expr<FXD>>>
    for Element<FuncDef, QueryDef, ProcDef, Expr<FXS>>
where
    FXS: Effect,
    FXD: Effect,
    FXD: TryFrom<FXS, Error = FXS>,
{
    fn restrict(
        self,
        span: Span,
    ) -> Result<Element<FuncDef, QueryDef, ProcDef, Expr<FXD>>, ChumskyError> {
        use Element::*;

        match self {
            Func(x) => Ok(Func(x)),
            Query(x) => Ok(Query(x)),
            Proc(x) => Ok(Proc(x)),
            Attr(rc_id, x) => x.restrict(span).map(|x| Attr(rc_id, x)),
        }
    }
}

/// Reused Containers:
impl<SL, SR, DL, DR> RestrictInto<LeftAssoc<DL, DR>> for LeftAssoc<SL, SR>
where
    SL: RestrictInto<DL>,
    SR: RestrictInto<DR>,
{
    fn restrict(self, span: Span) -> Result<LeftAssoc<DL, DR>, ChumskyError> {
        self.try_map(
            |left| left.restrict(span.clone()),
            |right| right.restrict(span.clone()),
        )
    }
}

/// Generic Plumbing:
impl<S, D> RestrictInto<Box<D>> for Box<S>
where
    S: RestrictInto<D>,
{
    fn restrict(self, span: Span) -> Result<Box<D>, ChumskyError> {
        (*self).restrict(span).map(Box::new)
    }
}
