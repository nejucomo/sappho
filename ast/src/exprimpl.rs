use crate::{Application, Expr, FuncExpr, LetExpr, ObjectExpr, Pattern};

impl Expr {
    pub fn let_expr(binding: Pattern, bindexpr: Expr, tail: Expr) -> Self {
        Expr::Let(LetExpr {
            binding,
            bindexpr: Box::new(bindexpr),
            tail: Box::new(tail),
        })
    }

    pub fn func_expr(binding: Pattern, body: Expr) -> Self {
        Expr::Func(FuncExpr {
            binding,
            body: Box::new(body),
        })
    }

    pub fn application(target: Expr, argument: Expr) -> Self {
        Expr::Apply(Application {
            target: Box::new(target),
            argument: Box::new(argument),
        })
    }

    pub fn object_expr(func: Option<(Pattern, Expr)>) -> Self {
        Expr::Object(ObjectExpr {
            func: func.map(|(binding, body)| FuncExpr {
                binding,
                body: Box::new(body),
            }),
        })
    }
}
