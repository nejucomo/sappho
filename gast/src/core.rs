use crate::{
    ApplicationExpr, EffectExpr, Identifier, LetExpr, Literal, LookupExpr, MatchExpr, ObjectDef,
};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq)]
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
