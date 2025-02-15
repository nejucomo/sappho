use std::ops::Deref;

use derive_new::new;
use sappho_ast_effect::Effect;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_object::Object;
use sappho_unparse::Unparse;

use crate::{Expr, FuncDef, ProcDef, QueryDef};

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Clone, Default, PartialEq, Debug, new)]
pub struct ObjectDef<FX>(Object<FuncDef, QueryDef, ProcDef, Expr<FX>>)
where
    FX: Effect;

impl<FX> ObjectDef<FX>
where
    FX: Effect,
{
    pub fn new_from_parts(
        f: Option<FuncDef>,
        q: Option<QueryDef>,
        p: Option<ProcDef>,
        attrs: IdentMap<Expr<FX>>,
    ) -> Self {
        Self::new(Object::new(f, q, p, attrs))
    }

    pub fn new_func(func: FuncDef) -> Self {
        ObjectDef(Object::new_func(func))
    }

    pub fn new_query(query: QueryDef) -> Self {
        ObjectDef(Object::new_query(query))
    }

    pub fn new_proc(proc: ProcDef) -> Self {
        ObjectDef(Object::new_proc(proc))
    }

    pub fn new_attrs<T>(attrs: T) -> Self
    where
        T: Into<IdentMap<Expr<FX>>>,
    {
        ObjectDef(Object::new_attrs(attrs))
    }

    pub fn unbundle(self) -> sappho_object::Unbundled<FuncDef, QueryDef, ProcDef, Expr<FX>> {
        self.0.unbundle()
    }

    pub fn into_try_map_values<F, FXD, E>(self, f: F) -> Result<ObjectDef<FXD>, E>
    where
        F: Fn(Expr<FX>) -> Result<Expr<FXD>, E>,
        FXD: Effect,
    {
        self.0.into_try_map_values(f).map(ObjectDef)
    }
}

impl<FX> Deref for ObjectDef<FX>
where
    FX: Effect,
{
    type Target = Object<FuncDef, QueryDef, ProcDef, Expr<FX>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<FX> AsRef<Object<FuncDef, QueryDef, ProcDef, Expr<FX>>> for ObjectDef<FX>
where
    FX: Effect,
{
    fn as_ref(&self) -> &Object<FuncDef, QueryDef, ProcDef, Expr<FX>> {
        &self.0
    }
}

impl<FX> TryIntoIdentMap<Expr<FX>> for ObjectDef<FX>
where
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<Expr<FX>>> {
        self.0.try_into_identmap()
    }
}

impl<FX> Unparse for ObjectDef<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}
