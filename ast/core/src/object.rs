use std::ops::Deref;

use derive_new::new;
use sappho_ast_effect::{Effect, ProcEffect, PureEffect, QueryEffect};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_object::Object;
use sappho_unparse::Unparse;

use crate::{ExprProvider, FuncDef, ProcDef, QueryDef};

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Debug, new)]
pub struct ObjectDef<XP, FX>(Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, XP::Expr<FX>>)
where
    XP: ExprProvider,
    FX: Effect;

impl<XP, FX> ObjectDef<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
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
        T: Into<IdentMap<XP::Expr<FX>>>,
    {
        ObjectDef(Object::new_attrs(attrs))
    }

    pub fn transform_into<XPD>(self) -> ObjectDef<XPD, FX>
    where
        XPD: ExprProvider,
        XPD::Pattern: From<XP::Pattern>,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
        XPD::Expr<PureEffect>: From<XP::Expr<PureEffect>>,
        XPD::Expr<QueryEffect>: From<XP::Expr<QueryEffect>>,
        XPD::Expr<ProcEffect>: From<XP::Expr<ProcEffect>>,
    {
        ObjectDef(self.transform_into_object())
    }

    pub fn transform_into_object<XPD>(
        self,
    ) -> Object<FuncDef<XPD>, QueryDef<XPD>, ProcDef<XPD>, XPD::Expr<FX>>
    where
        XPD: ExprProvider,
        XPD::Pattern: From<XP::Pattern>,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
        XPD::Expr<PureEffect>: From<XP::Expr<PureEffect>>,
        XPD::Expr<QueryEffect>: From<XP::Expr<QueryEffect>>,
        XPD::Expr<ProcEffect>: From<XP::Expr<ProcEffect>>,
    {
        self.0.transform(
            |func| func.transform_into(),
            |query| query.transform_into(),
            |proc| proc.transform_into(),
            XPD::Expr::<FX>::from,
        )
    }

    pub fn unbundle(
        self,
    ) -> sappho_object::Unbundled<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, XP::Expr<FX>> {
        self.0.unbundle()
    }
}

impl<XP, FX> Default for ObjectDef<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn default() -> Self {
        ObjectDef(Object::default())
    }
}

impl<XP, FX> Deref for ObjectDef<XP, FX>
where
    XP: ExprProvider,
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
    XP: ExprProvider,
    FX: Effect,
{
    fn as_ref(&self) -> &Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, XP::Expr<FX>> {
        &self.0
    }
}

impl<XP, FX> TryIntoIdentMap<XP::Expr<FX>> for ObjectDef<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<XP::Expr<FX>>> {
        self.0.try_into_identmap()
    }
}

impl<XP, FX> Unparse for ObjectDef<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}

impl<XP, FX> Clone for ObjectDef<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        ObjectDef(self.0.clone())
    }
}

impl<XP, FX> PartialEq for ObjectDef<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
