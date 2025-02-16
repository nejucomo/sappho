use derive_new::new;
use either::Either;
use sappho_ast_core::Effect;
use sappho_listform::ListForm;
use sappho_unparse::Unparse;

use crate::Expr;

// TODO: Remove this type and use `ListForm` directly (maybe with alias).
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

impl<FX> FromIterator<Either<Expr<FX>, Box<Expr<FX>>>> for ListExpr<FX>
where
    FX: Effect,
{
    fn from_iter<I: IntoIterator<Item = Either<Expr<FX>, Box<Expr<FX>>>>>(iter: I) -> Self {
        ListExpr(iter.into_iter().collect())
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
