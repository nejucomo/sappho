use either::Either::{Left, Right};
use sappho_ast_effect::{Effect, EffectDescription, ProcEffect, PureEffect, QueryEffect};
use sappho_listform::ListForm;
use sappho_object::{Element, Object};
use sappho_parsable::error::{ChumskyError, Span};

use crate::leftassoc::LeftAssoc;
use crate::spanned::Spanned;
use crate::{
    Application, Applications, Confined, EffectExpr, Expr, FuncDef, Let, LetClause, Lookup,
    Lookups, Match, MatchClause, ParensExpr, ProcDef, ProcExpr, PureExpr, QueryDef, QueryExpr, BSE,
    SE,
};

// Restriction converts ProcEffects downwards or produces an error. This happens during parsing because it's difficult to parse all three effects recursions directly, so we restrict inside the parser.
pub(crate) trait RestrictInto<D> {
    fn restrict(self, span: Span) -> Result<D, ChumskyError>;
}

// Restricting effects exprs:
impl RestrictInto<QueryEffect> for ProcEffect {
    fn restrict(self, span: Span) -> Result<QueryEffect, ChumskyError> {
        QueryEffect::try_from(self).map_err(|pfx| make_error(span, QueryEffect::context(), pfx))
    }
}

impl RestrictInto<PureEffect> for ProcEffect {
    fn restrict(self, span: Span) -> Result<PureEffect, ChumskyError> {
        PureEffect::try_from(self).map_err(|pfx| make_error(span, PureEffect::context(), pfx))
    }
}

fn make_error(span: Span, context: &'static str, pfx: ProcEffect) -> ChumskyError {
    let EffectDescription { noun, sigil, .. } = pfx.description();
    ChumskyError::custom(
        span,
        format!("{noun} {sigil:?} restricted in {context} contexts"),
    )
}

// Location tracking in restriction
impl<T, S> RestrictInto<Spanned<T>> for Spanned<S>
where
    S: RestrictInto<T>,
{
    fn restrict(self, _: Span) -> Result<Spanned<T>, ChumskyError> {
        // We shadow the outer span with the new source span:
        self.node
            .restrict(self.span.clone())
            .map(|fxd| Spanned::new(fxd, self.span))
    }
}

// Restricting top-level exprs:
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

impl RestrictInto<ProcExpr> for ProcExpr {
    fn restrict(self, _: Span) -> Result<ProcExpr, ChumskyError> {
        Ok(self)
    }
}

// Top-Level Recursion Nexus
impl<FX> RestrictInto<BSE<FX>> for BSE<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<BSE<FX>, ChumskyError> {
        self.0.restrict(span).map(BSE)
    }
}

impl<FX> RestrictInto<SE<FX>> for SE<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<SE<FX>, ChumskyError> {
        self.0.restrict(span).map(SE)
    }
}

// FX-generic restrictions:
impl<FX> RestrictInto<Expr<FX>> for Expr<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<Expr<FX>, ChumskyError> {
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

impl<FX> RestrictInto<Let<FX>> for Let<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<Let<FX>, ChumskyError> {
        let clauses = self
            .clauses
            .into_iter()
            .map(|clause| clause.restrict(span.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let inner = self.inner.restrict(span)?;

        Ok(Let { clauses, inner })
    }
}

impl<FX> RestrictInto<LetClause<FX>> for LetClause<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<LetClause<FX>, ChumskyError> {
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

impl<FX> RestrictInto<Match<FX>> for Match<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<Match<FX>, ChumskyError> {
        let clauses = self
            .clauses
            .into_iter()
            .map(|clause| clause.restrict(span.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let candidate = self.candidate.restrict(span)?;

        Ok(Match { candidate, clauses })
    }
}

impl<FX> RestrictInto<MatchClause<FX>> for MatchClause<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<MatchClause<FX>, ChumskyError> {
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

impl<FX> RestrictInto<Applications<FX>> for Applications<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<Applications<FX>, ChumskyError> {
        self.0.restrict(span).map(Applications)
    }
}

impl<FX> RestrictInto<Application<FX>> for Application<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<Application<FX>, ChumskyError> {
        self.0.restrict(span).map(Application)
    }
}

impl<FX> RestrictInto<Lookups<FX>> for Lookups<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<Lookups<FX>, ChumskyError> {
        self.0.restrict(span).map(Lookups)
    }
}

impl RestrictInto<Lookup> for Lookup {
    fn restrict(self, _: Span) -> Result<Lookup, ChumskyError> {
        Ok(self)
    }
}

impl<FX> RestrictInto<EffectExpr<FX>> for EffectExpr<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<EffectExpr<FX>, ChumskyError> {
        let effects = self
            .effects
            .into_iter()
            .map(|fx| fx.restrict(span.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let confined = self.confined.restrict(span)?;
        Ok(EffectExpr { effects, confined })
    }
}

impl<FX> RestrictInto<Confined<FX>> for Confined<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<Confined<FX>, ChumskyError> {
        use Confined::*;

        match self {
            Ref(x) => Ok(Ref(x)),
            Prim(x) => Ok(Prim(x)),
            Parens(x) => x.restrict(span).map(Parens),
            ObjectDef(x) => x.restrict(span).map(ObjectDef),
            ListExpr(x) => x.restrict(span).map(ListExpr),
        }
    }
}

impl<FX> RestrictInto<ParensExpr<FX>> for ParensExpr<ProcEffect>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<ParensExpr<FX>, ChumskyError> {
        self.0.restrict(span).map(ParensExpr)
    }
}

impl<FX> RestrictInto<Object<FuncDef, QueryDef, ProcDef, SE<FX>>>
    for Object<FuncDef, QueryDef, ProcDef, SE<ProcEffect>>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(
        self,
        span: Span,
    ) -> Result<Object<FuncDef, QueryDef, ProcDef, SE<FX>>, ChumskyError> {
        self.into_iter()
            .map(|x| x.restrict(span.clone()))
            .collect::<Result<Result<Object<_, _, _, _>, String>, ChumskyError>>()
            .map(|res: Result<_, String>| res.unwrap())
    }
}

impl<FX> RestrictInto<Element<FuncDef, QueryDef, ProcDef, SE<FX>>>
    for Element<FuncDef, QueryDef, ProcDef, SE<ProcEffect>>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(
        self,
        span: Span,
    ) -> Result<Element<FuncDef, QueryDef, ProcDef, SE<FX>>, ChumskyError> {
        use Element::*;

        match self {
            Func(x) => Ok(Func(x)),
            Query(x) => Ok(Query(x)),
            Proc(x) => Ok(Proc(x)),
            Attr(rc_id, x) => x.restrict(span).map(|x| Attr(rc_id, x)),
        }
    }
}

impl<FX> RestrictInto<ListForm<SE<FX>, BSE<FX>>> for ListForm<SE<ProcEffect>, BSE<ProcEffect>>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn restrict(self, span: Span) -> Result<ListForm<SE<FX>, BSE<FX>>, ChumskyError> {
        self.into_iter()
            .map(|ei| {
                ei.either(
                    |x| x.restrict(span.clone()).map(Left),
                    |bx| bx.restrict(span.clone()).map(Right),
                )
            })
            .collect()
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
