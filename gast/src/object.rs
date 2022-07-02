use crate::{FuncDef, QueryDef};
use sappho_identmap::IdentMap;
use sappho_object::Object;
use std::fmt;

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectDef<PureExpr, QueryExpr>(ObjectInner<PureExpr, QueryExpr>);
pub type ObjectInner<PureExpr, QueryExpr> =
    Object<FuncDef<PureExpr>, QueryDef<QueryExpr>, PureExpr>;

#[derive(Debug)]
pub enum Unbundled<P, Q> {
    Bundled(ObjectDef<P, Q>),
    Func(FuncDef<P>),
    Query(QueryDef<Q>),
}

impl<P, Q> std::ops::Deref for ObjectDef<P, Q> {
    type Target = ObjectInner<P, Q>;

    fn deref(&self) -> &ObjectInner<P, Q> {
        &self.0
    }
}

impl<P, Q> ObjectDef<P, Q> {
    pub fn new(func: Option<FuncDef<P>>, query: Option<QueryDef<Q>>, attrs: IdentMap<P>) -> Self {
        ObjectDef(ObjectInner::new(func, query, attrs))
    }

    pub fn new_func(func: FuncDef<P>) -> Self {
        ObjectDef(ObjectInner::new_func(func))
    }

    pub fn new_query(query: QueryDef<Q>) -> Self {
        ObjectDef(ObjectInner::new_query(query))
    }

    pub fn new_attrs(attrs: IdentMap<P>) -> Self {
        ObjectDef(ObjectInner::new_attrs(attrs))
    }

    pub fn transform_into<P2, Q2>(self) -> ObjectDef<P2, Q2>
    where
        P2: From<P>,
        Q2: From<Q>,
    {
        ObjectDef(self.0.transform(
            |func| func.transform_into(),
            |query| query.transform_into(),
            P2::from,
        ))
    }

    pub fn unbundle(self) -> Unbundled<P, Q> {
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

impl<P, Q> fmt::Display for ObjectDef<P, Q>
where
    P: fmt::Display,
    Q: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
