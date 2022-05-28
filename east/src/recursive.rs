use crate::{AstFxFor, FromFx, GenExpr, Pattern};
use sappho_ast as ast;

#[derive(Debug, PartialEq)]
pub enum RecursiveExpr<Effects> {
    List(Vec<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Apply(Application<Effects>),
}

impl<FX> From<ast::RecursiveExpr<AstFxFor<FX>>> for RecursiveExpr<FX>
where
    FX: FromFx,
{
    fn from(re: ast::RecursiveExpr<AstFxFor<FX>>) -> Self {
        use RecursiveExpr::*;

        match re {
            ast::RecursiveExpr::List(x) => List(x.into_iter().map(GenExpr::from).collect()),
            ast::RecursiveExpr::Let(x) => Let(LetExpr::from(x)),
            ast::RecursiveExpr::Apply(x) => Apply(Application::from(x)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetExpr<Effects> {
    pub binding: Pattern,
    pub bindexpr: Box<GenExpr<Effects>>,
    pub tail: Box<GenExpr<Effects>>,
}

impl<FX> From<ast::LetExpr<AstFxFor<FX>>> for LetExpr<FX>
where
    FX: FromFx,
{
    fn from(re: ast::LetExpr<AstFxFor<FX>>) -> Self {
        LetExpr {
            binding: re.binding,
            bindexpr: Box::new(GenExpr::from(*re.bindexpr)),
            tail: Box::new(GenExpr::from(*re.tail)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Application<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub argument: Box<GenExpr<Effects>>,
}

impl<FX> From<ast::Application<AstFxFor<FX>>> for Application<FX>
where
    FX: FromFx,
{
    fn from(app: ast::Application<AstFxFor<FX>>) -> Self {
        Application {
            target: Box::new(GenExpr::from(*app.target)),
            argument: Box::new(GenExpr::from(*app.argument)),
        }
    }
}
