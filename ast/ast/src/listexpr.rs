use derive_new::new;
use either::Either;
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

impl<FX> IntoIterator for ListExpr<FX>
where
    FX: Effect,
{
    type Item = Either<Expr<FX>, Box<Expr<FX>>>;
    type IntoIter = <ListForm<Expr<FX>, Box<Expr<FX>>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
