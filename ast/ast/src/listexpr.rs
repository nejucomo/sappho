use derive_new::new;
use sappho_ast_core::Effect;
use sappho_listform::ListForm;
use sappho_unparse::Unparse;

use crate::Expr;

#[derive(Clone, Debug, PartialEq, new)]
pub struct ListExpr<FX>(ListForm<Expr<FX>, Box<Expr<FX>>>)
where
    FX: Effect;

impl<FX> ListExpr<FX>
where
    FX: Effect,
{
    pub fn new_from_parts<T>(iter: T, optail: Option<Box<Expr<FX>>>) -> Self
    where
        T: IntoIterator<Item = Expr<FX>>,
    {
        Self::new(ListForm::new(iter, optail))
    }

    pub fn into_reverse_fold<S, TT, F>(self, ttail: TT, f: F) -> S
    where
        TT: FnOnce(Option<Box<Expr<FX>>>) -> S,
        F: Fn(S, Expr<FX>) -> S,
    {
        self.0.into_reverse_fold(ttail, f)
    }

    pub fn map_elems<F, DX>(self, f: F) -> ListForm<DX, Box<Expr<FX>>>
    where
        F: Fn(Expr<FX>) -> DX,
        DX: std::fmt::Debug,
    {
        self.0.map_elems(f)
    }

    pub fn try_map<F, FXD, E>(self, f: F) -> Result<ListExpr<FXD>, E>
    where
        F: Fn(Expr<FX>) -> Result<Expr<FXD>, E>,
        FXD: Effect,
    {
        self.0
            .try_map(&f, |tail| f(*tail).map(Box::new))
            .map(ListExpr::new)
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
