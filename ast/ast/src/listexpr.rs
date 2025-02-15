use derive_new::new;
use sappho_ast_core::{
    AstCore, AstTransformInto, BoxExpr, CommentedExpr, CoreExpr, Effect, ObjectDef,
};
use sappho_identmap::HeadTailUnrollable;
use sappho_listform::ListForm;
use sappho_unparse::Unparse;

use crate::Ast;

#[derive(Clone, Debug, PartialEq, new)]
pub struct ListExpr<FX>(ListForm<CommentedExpr<Ast, FX>, BoxExpr<Ast, FX>>)
where
    FX: Effect;

impl<FX> ListExpr<FX>
where
    FX: Effect,
{
    pub fn new_from_parts<T>(iter: T, optail: Option<BoxExpr<Ast, FX>>) -> Self
    where
        T: IntoIterator<Item = CommentedExpr<Ast, FX>>,
    {
        Self::new(ListForm::new(iter, optail))
    }

    pub fn into_reverse_fold<S, TT, F>(self, ttail: TT, f: F) -> S
    where
        TT: FnOnce(Option<BoxExpr<Ast, FX>>) -> S,
        F: Fn(S, CommentedExpr<Ast, FX>) -> S,
    {
        self.0.into_reverse_fold(ttail, f)
    }

    pub fn map_elems<F, DX>(self, f: F) -> ListForm<DX, BoxExpr<Ast, FX>>
    where
        F: Fn(CommentedExpr<Ast, FX>) -> DX,
    {
        self.0.map_elems(f)
    }

    pub fn try_map<F, FXD, E>(self, f: F) -> Result<ListExpr<FXD>, E>
    where
        F: Fn(CommentedExpr<Ast, FX>) -> Result<CommentedExpr<Ast, FXD>, E>,
        FXD: Effect,
    {
        self.0
            .try_map(&f, |tail| f(tail.into_inner()).map(BoxExpr::from))
            .map(ListExpr::new)
    }
}

impl<FX> AstTransformInto<CommentedExpr<AstCore, FX>> for ListExpr<FX>
where
    FX: Effect,
{
    fn ast_transform(self) -> CommentedExpr<AstCore, FX> {
        self.0.unroll(
            |cmtexpr| cmtexpr.ast_transform(),
            |boxexpr| boxexpr.into_inner().ast_transform(),
            |idmap| CommentedExpr::new_bare(CoreExpr::Object(ObjectDef::new_attrs(idmap))),
        )
    }
}

impl<FX> Unparse for ListExpr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}
