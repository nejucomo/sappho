use derive_new::new;
use sappho_ast_effect::Effect;
use sappho_listform::ListForm;
use sappho_unparse::Unparse;

use crate::CoreExpr;

#[derive(Clone, Debug, PartialEq, new)]
pub struct ListExpr<FX>(ListForm<CoreExpr<FX>, Box<CoreExpr<FX>>>)
where
    FX: Effect;

impl<FX> ListExpr<FX>
where
    FX: Effect,
{
    pub fn new_from_parts<T>(iter: T, optail: Option<Box<CoreExpr<FX>>>) -> Self
    where
        T: IntoIterator<Item = CoreExpr<FX>>,
    {
        Self::new(ListForm::new(iter, optail))
    }

    pub fn into_reverse_fold<S, TT, F>(self, ttail: TT, f: F) -> S
    where
        TT: FnOnce(Option<Box<CoreExpr<FX>>>) -> S,
        F: Fn(S, CoreExpr<FX>) -> S,
    {
        self.0.into_reverse_fold(ttail, f)
    }

    pub fn map_elems<F, DX>(self, f: F) -> ListForm<DX, Box<CoreExpr<FX>>>
    where
        F: Fn(CoreExpr<FX>) -> DX,
    {
        self.0.map_elems(f)
    }

    pub fn try_map<F, FXD, E>(self, f: F) -> Result<ListExpr<FXD>, E>
    where
        F: Fn(CoreExpr<FX>) -> Result<CoreExpr<FXD>, E>,
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
