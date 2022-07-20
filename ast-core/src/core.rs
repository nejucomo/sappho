use crate::{
    ApplicationExpr, EffectExpr, Identifier, LetExpr, Literal, LookupExpr, MatchExpr, ObjectDef,
};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum CoreExpr<Pattern, PureExpr, QueryExpr, Expr, FX> {
    Lit(Literal),
    Ref(Identifier),
    Object(ObjectDef<Pattern, PureExpr, QueryExpr, Expr>),
    Let(LetExpr<Pattern, Expr>),
    Match(MatchExpr<Pattern, Expr>),
    Application(ApplicationExpr<Expr>),
    Lookup(LookupExpr<Expr>),
    Effect(EffectExpr<FX, Expr>),
}

impl<P, X, Q, G, FX> CoreExpr<P, X, Q, G, FX> {
    pub fn transform_into<PD, XD, QD, GD>(self) -> CoreExpr<PD, XD, QD, GD, FX>
    where
        PD: From<P>,
        XD: From<X>,
        QD: From<Q>,
        GD: From<G>,
    {
        use CoreExpr::*;

        match self {
            Lit(x) => Lit(x),
            Ref(x) => Ref(x),
            Object(x) => Object(crate::transform_object_def(x)),
            Let(x) => Let(x.transform_into()),
            Match(x) => Match(x.transform_into()),
            Application(x) => Application(x.transform_into()),
            Lookup(x) => Lookup(x.transform_into()),
            Effect(x) => Effect(x.transform_into()),
        }
    }
}

impl<P, X, Q, G, FX> TryIntoIdentMap<G> for CoreExpr<P, X, Q, G, FX> {
    fn try_into_identmap(&self) -> Option<&IdentMap<G>> {
        match self {
            CoreExpr::Object(objdef) => objdef.try_into_identmap(),
            _ => None,
        }
    }
}

impl<P, X, Q, G, FX> Unparse for CoreExpr<P, X, Q, G, FX>
where
    P: Unparse,
    X: Unparse,
    Q: Unparse,
    G: Unparse,
    FX: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use CoreExpr::*;

        match self {
            Lit(x) => x.unparse_into(s),
            Ref(x) => x.unparse_into(s),
            Object(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            Lookup(x) => x.unparse_into(s),
            Effect(x) => x.unparse_into(s),
        }
    }
}
