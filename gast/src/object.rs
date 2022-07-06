use crate::{FuncDef, QueryDef};
use sappho_identmap::IdentMap;
use sappho_object::Object;
use std::fmt;

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectDef<Pattern, PureExpr, QueryExpr, Expr>(
    ObjectInner<Pattern, PureExpr, QueryExpr, Expr>,
);
pub type ObjectInner<Pattern, PureExpr, QueryExpr, Expr> =
    Object<FuncDef<Pattern, PureExpr>, QueryDef<QueryExpr>, Expr>;

#[derive(Debug)]
pub enum Unbundled<P, X, Q, G> {
    Bundled(ObjectDef<P, X, Q, G>),
    Func(FuncDef<P, X>),
    Query(QueryDef<Q>),
}

impl<P, X, Q, G> std::ops::Deref for ObjectDef<P, X, Q, G> {
    type Target = ObjectInner<P, X, Q, G>;

    fn deref(&self) -> &ObjectInner<P, X, Q, G> {
        &self.0
    }
}

impl<P, X, Q, G> Default for ObjectDef<P, X, Q, G> {
    fn default() -> Self {
        ObjectDef(ObjectInner::default())
    }
}

impl<P, X, Q, G> ObjectDef<P, X, Q, G> {
    pub fn new(
        func: Option<FuncDef<P, X>>,
        query: Option<QueryDef<Q>>,
        attrs: IdentMap<G>,
    ) -> Self {
        ObjectDef(ObjectInner::new(func, query, attrs))
    }

    pub fn new_func(func: FuncDef<P, X>) -> Self {
        ObjectDef(ObjectInner::new_func(func))
    }

    pub fn new_query(query: QueryDef<Q>) -> Self {
        ObjectDef(ObjectInner::new_query(query))
    }

    pub fn new_attrs<T>(attrs: T) -> Self
    where
        T: Into<IdentMap<G>>,
    {
        ObjectDef(ObjectInner::new_attrs(attrs))
    }

    pub fn transform_into<PD, XD, QD, GD>(self) -> ObjectDef<PD, XD, QD, GD>
    where
        PD: From<P>,
        XD: From<X>,
        QD: From<Q>,
        GD: From<G>,
    {
        ObjectDef(self.0.transform(
            |func| func.transform_into(),
            |query| query.transform_into(),
            GD::from,
        ))
    }

    pub fn unwrap(self) -> (Option<FuncDef<P, X>>, Option<QueryDef<Q>>, IdentMap<G>) {
        self.0.unwrap()
    }

    pub fn unbundle(self) -> Unbundled<P, X, Q, G> {
        use sappho_object::Unbundled as OU;
        use Unbundled::*;

        match self.0.unbundle() {
            OU::Bundled(inner) => Bundled(ObjectDef(inner)),
            OU::Func(f) => Func(f),
            OU::Query(q) => Query(q),
            OU::Attrs(a) => Bundled(ObjectDef::new_attrs(a)),
        }
    }
}

impl<P, X, Q, G> fmt::Display for ObjectDef<P, X, Q, G>
where
    P: fmt::Display,
    X: fmt::Display,
    Q: fmt::Display,
    G: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
