use crate::{
    Application, FuncClause, GenExpr, LetExpr, ObjectExpr, ProcEffects, PureEffects, QueryClause,
    QueryEffects,
};
use saplang_ast as ast;

pub trait FromEffects {
    type ASTEffects;

    fn from_effects(fx: Self::ASTEffects) -> Self;
}

type AstFxFor<FX> = <FX as FromEffects>::ASTEffects;

impl FromEffects for PureEffects {
    type ASTEffects = Self;

    fn from_effects(fx: Self) -> Self {
        fx
    }
}

impl FromEffects for QueryEffects {
    type ASTEffects = ast::QueryEffects;

    fn from_effects(fx: ast::QueryEffects) -> QueryEffects {
        use ast::QueryEffects::Inquire as ASTInquire;
        use QueryEffects::*;

        match fx {
            ASTInquire(bx) => Inquire(box_expr_from(bx)),
        }
    }
}

impl FromEffects for ProcEffects {
    type ASTEffects = ast::ProcEffects;

    fn from_effects(fx: ast::ProcEffects) -> ProcEffects {
        use ast::ProcEffects::{Evoke as ASTEvoke, Inquire as ASTInquire};
        use ProcEffects::*;

        match fx {
            ASTInquire(bx) => Inquire(box_expr_from(bx)),
            ASTEvoke(bx) => Evoke(box_expr_from(bx)),
        }
    }
}

impl<FX> From<ast::GenExpr<AstFxFor<FX>>> for GenExpr<FX>
where
    FX: FromEffects,
{
    fn from(ae: ast::GenExpr<AstFxFor<FX>>) -> GenExpr<FX> {
        use GenExpr::*;

        match ae {
            ast::GenExpr::Lit(x) => Lit(x),
            ast::GenExpr::Ref(x) => Ref(x),
            ast::GenExpr::List(x) => List(x.into_iter().map(GenExpr::from).collect()),
            ast::GenExpr::Let(x) => Let(x.into()),
            ast::GenExpr::Func(x) => Object(x.into()),
            ast::GenExpr::Apply(x) => Apply(x.into()),
            ast::GenExpr::Query(x) => Object(x.into()),
            ast::GenExpr::Object(x) => Object(x.into()),
            ast::GenExpr::Effect(x) => Effect(FX::from_effects(x)),
        }
    }
}

impl<FX> From<ast::LetExpr<AstFxFor<FX>>> for LetExpr<FX>
where
    FX: FromEffects,
{
    fn from(ale: ast::LetExpr<AstFxFor<FX>>) -> LetExpr<FX> {
        LetExpr {
            binding: ale.binding,
            bindexpr: box_expr_from(ale.bindexpr),
            tail: box_expr_from(ale.tail),
        }
    }
}

impl<FX> From<ast::Application<AstFxFor<FX>>> for Application<FX>
where
    FX: FromEffects,
{
    fn from(aa: ast::Application<AstFxFor<FX>>) -> Application<FX> {
        Application {
            target: box_expr_from(aa.target),
            argument: box_expr_from(aa.argument),
        }
    }
}

impl From<ast::ObjectExpr> for ObjectExpr {
    fn from(ao: ast::ObjectExpr) -> ObjectExpr {
        ObjectExpr {
            query: ao.query.map(QueryClause::from),
            func: ao.func.map(FuncClause::from),
        }
    }
}

impl From<ast::FuncExpr> for ObjectExpr {
    fn from(fe: ast::FuncExpr) -> ObjectExpr {
        ObjectExpr {
            query: None,
            func: Some(FuncClause::from(fe)),
        }
    }
}

impl From<ast::FuncExpr> for FuncClause {
    fn from(fe: ast::FuncExpr) -> FuncClause {
        FuncClause {
            binding: fe.binding,
            body: std::rc::Rc::new(GenExpr::from(*fe.body)),
        }
    }
}

impl From<ast::QueryExpr> for ObjectExpr {
    fn from(fe: ast::QueryExpr) -> ObjectExpr {
        ObjectExpr {
            query: Some(QueryClause::from(fe)),
            func: None,
        }
    }
}

impl From<ast::QueryExpr> for QueryClause {
    fn from(fe: ast::QueryExpr) -> QueryClause {
        QueryClause {
            body: std::rc::Rc::new(GenExpr::from(*fe.body)),
        }
    }
}

#[allow(clippy::boxed_local)]
fn box_expr_from<FX>(b: Box<ast::GenExpr<AstFxFor<FX>>>) -> Box<GenExpr<FX>>
where
    FX: FromEffects,
{
    Box::new(GenExpr::from(*b))
}
