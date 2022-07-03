use crate::{FuncDef, QueryDef};
use sappho_identmap::IdentMap;
use sappho_object::Object;
use std::fmt;

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectDef<Pattern, PureExpr, QueryExpr>(ObjectInner<Pattern, PureExpr, QueryExpr>);
pub type ObjectInner<Pattern, PureExpr, QueryExpr> =
    Object<FuncDef<Pattern, PureExpr>, QueryDef<QueryExpr>, PureExpr>;

#[derive(Debug)]
pub enum Unbundled<P, X, Q> {
    Bundled(ObjectDef<P, X, Q>),
    Func(FuncDef<P, X>),
    Query(QueryDef<Q>),
}

impl<P, X, Q> std::ops::Deref for ObjectDef<P, X, Q> {
    type Target = ObjectInner<P, X, Q>;

    fn deref(&self) -> &ObjectInner<P, X, Q> {
        &self.0
    }
}

impl<P, X, Q> ObjectDef<P, X, Q> {
    pub fn new(
        func: Option<FuncDef<P, X>>,
        query: Option<QueryDef<Q>>,
        attrs: IdentMap<X>,
    ) -> Self {
        ObjectDef(ObjectInner::new(func, query, attrs))
    }

    pub fn new_func(func: FuncDef<P, X>) -> Self {
        ObjectDef(ObjectInner::new_func(func))
    }

    pub fn new_query(query: QueryDef<Q>) -> Self {
        ObjectDef(ObjectInner::new_query(query))
    }

    pub fn new_attrs(attrs: IdentMap<X>) -> Self {
        ObjectDef(ObjectInner::new_attrs(attrs))
    }

    pub fn transform_into<PD, PX, QD>(self) -> ObjectDef<PD, PX, QD>
    where
        PD: From<P>,
        PX: From<X>,
        QD: From<Q>,
    {
        ObjectDef(self.0.transform(
            |func| func.transform_into(),
            |query| query.transform_into(),
            PX::from,
        ))
    }

    pub fn unbundle(self) -> Unbundled<P, X, Q> {
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

impl<P, X, Q> fmt::Display for ObjectDef<P, X, Q>
where
    P: fmt::Display,
    X: fmt::Display,
    Q: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
