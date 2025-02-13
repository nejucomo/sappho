use std::ops::Deref;

use derive_new::new;
use sappho_ast_effect::Effect;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_object::Object;
use sappho_unparse::Unparse;

use crate::{AstProvider, AstTransformInto, CommentedExpr, FuncDef, ProcDef, QueryDef};

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Debug, new)]
pub struct ObjectDef<XP, FX>(Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, CommentedExpr<XP, FX>>)
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
        attrs: IdentMap<CommentedExpr<XP, FX>>,
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
        T: Into<IdentMap<CommentedExpr<XP, FX>>>,
    {
        ObjectDef(Object::new_attrs(attrs))
    }

    pub fn unbundle(
        self,
    ) -> sappho_object::Unbundled<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, CommentedExpr<XP, FX>>
    {
        self.0.unbundle()
    }

    pub fn into_try_map_values<F, FXD, E>(self, f: F) -> Result<ObjectDef<XP, FXD>, E>
    where
        F: Fn(CommentedExpr<XP, FX>) -> Result<CommentedExpr<XP, FXD>, E>,
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

impl<XPS, XPD, FX> AstTransformInto<ObjectDef<XPD, FX>> for ObjectDef<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FuncDef<XPS>: AstTransformInto<FuncDef<XPD>>,
    QueryDef<XPS>: AstTransformInto<QueryDef<XPD>>,
    ProcDef<XPS>: AstTransformInto<ProcDef<XPD>>,
    FX: Effect,
{
    fn ast_transform(self) -> ObjectDef<XPD, FX> {
        ObjectDef(self.0.ast_transform())
    }
}

impl<FD, QD, PD, AD, FS, QS, PS, AS> AstTransformInto<Object<FD, QD, PD, AD>>
    for Object<FS, QS, PS, AS>
where
    FS: AstTransformInto<FD>,
    QS: AstTransformInto<QD>,
    PS: AstTransformInto<PD>,
    AS: AstTransformInto<AD>,
{
    fn ast_transform(self) -> Object<FD, QD, PD, AD> {
        self.transform(
            FS::ast_transform,
            QS::ast_transform,
            PS::ast_transform,
            AS::ast_transform,
        )
    }
}

impl<XP, FX> Deref for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    type Target = Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, CommentedExpr<XP, FX>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<XP, FX> AsRef<Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, CommentedExpr<XP, FX>>>
    for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn as_ref(&self) -> &Object<FuncDef<XP>, QueryDef<XP>, ProcDef<XP>, CommentedExpr<XP, FX>> {
        &self.0
    }
}

impl<XP, FX> TryIntoIdentMap<CommentedExpr<XP, FX>> for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<CommentedExpr<XP, FX>>> {
        self.0.try_into_identmap()
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
