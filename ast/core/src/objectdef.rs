use std::ops::Deref;

use derive_more::{From, Into};
use derive_new::new;
use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_object::Object;
use sappho_unparse::Unparse;

use crate::{AstProvider, FuncDef, ProcDef, QueryDef};

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Debug, new, From, Into)]
pub struct ObjectDef<XP, FX>(Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, XP::Expr<FX>>)
where
    XP: AstProvider,
    FX: Effect;

impl<XP, FX> ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn new_from_parts(
        f: Option<FuncDef<XP>>,
        q: Option<QueryDef<XP>>,
        p: Option<ProcDef<XP>>,
        attrs: Attrs<XP::Expr<FX>>,
    ) -> Self {
        Self::new(Object::new(f, q, p, attrs))
    }

    pub fn new_func(func: FuncDef<XP>) -> Self {
        ObjectDef(Object::new_func(func))
    }

    pub fn new_query(query: QueryDef<XP>) -> Self {
        ObjectDef(Object::new_query(query))
    }

    pub fn new_proc(proc: ProcDef<XP>) -> Self {
        ObjectDef(Object::new_proc(proc))
    }

    pub fn new_attrs<T>(attrs: T) -> Self
    where
        T: Into<Attrs<XP::Expr<FX>>>,
    {
        ObjectDef(Object::new_attrs(attrs))
    }

    pub fn unbundle(
        self,
    ) -> sappho_object::Unbundled<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, XP::Expr<FX>> {
        self.0.unbundle()
    }

    pub fn into_try_map_values<F, FXD, E>(self, f: F) -> Result<ObjectDef<XP, FXD>, E>
    where
        F: Fn(XP::Expr<FX>) -> Result<XP::Expr<FXD>, E>,
        FXD: Effect,
    {
        self.0.into_try_map_values(f).map(ObjectDef)
    }
}

impl<XP, FX> Default for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn default() -> Self {
        ObjectDef(Object::default())
    }
}

impl<XP, FX> From<FuncDef<XP>> for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: FuncDef<XP>) -> Self {
        ObjectDef(Object::new_func(value))
    }
}

impl<XP, FX> From<QueryDef<XP>> for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: QueryDef<XP>) -> Self {
        ObjectDef(Object::new_query(value))
    }
}

impl<XP, FX> From<ProcDef<XP>> for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: ProcDef<XP>) -> Self {
        ObjectDef(Object::new_proc(value))
    }
}

impl<XP, FX> From<Attrs<XP::Expr<FX>>> for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: Attrs<XP::Expr<FX>>) -> Self {
        ObjectDef(Object::new_attrs(value))
    }
}

impl<XP, FX> Deref for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    type Target = Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, XP::Expr<FX>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<XP, FX> AsRef<Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, XP::Expr<FX>>>
    for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn as_ref(&self) -> &Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, XP::Expr<FX>> {
        &self.0
    }
}

impl<XP, FX> Unparse for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}

impl<XP, FX> Clone for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        ObjectDef(self.0.clone())
    }
}

impl<XP, FX> PartialEq for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
