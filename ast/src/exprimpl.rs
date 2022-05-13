use crate::{Application, Expr, FuncExpr, LetExpr, Pattern};

impl Expr {
    pub fn let_expr(binding: Pattern, bindexpr: Expr, tail: Expr) -> Self {
        Expr::Let(LetExpr {
            binding,
            bindexpr: Box::new(bindexpr),
            tail: Box::new(tail),
        })
    }

    pub fn application(target: Expr, argument: Expr) -> Self {
        Expr::Apply(Application {
            target: Box::new(target),
            argument: Box::new(argument),
        })
    }

    pub fn func_expr(binding: Pattern, body: Expr) -> Self {
        Expr::Func(FuncExpr {
            binding,
            body: std::rc::Rc::new(body),
        })
    }
}
