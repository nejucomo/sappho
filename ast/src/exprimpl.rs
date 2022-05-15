use crate::{
    Application, Expr, FuncExpr, GenExpr, LetExpr, ObjectExpr, Pattern, QueryEffects, QueryExpr,
};

impl<FX> GenExpr<FX> {
    pub fn let_expr(binding: Pattern, bindexpr: GenExpr<FX>, tail: GenExpr<FX>) -> Self {
        GenExpr::Let(LetExpr {
            binding,
            bindexpr: Box::new(bindexpr),
            tail: Box::new(tail),
        })
    }

    pub fn func_expr(binding: Pattern, body: Expr) -> Self {
        GenExpr::Func(FuncExpr {
            binding,
            body: Box::new(body),
        })
    }

    pub fn application(target: GenExpr<FX>, argument: GenExpr<FX>) -> Self {
        GenExpr::Apply(Application {
            target: Box::new(target),
            argument: Box::new(argument),
        })
    }

    pub fn query_expr(body: GenExpr<QueryEffects>) -> Self {
        GenExpr::Query(QueryExpr {
            body: Box::new(body),
        })
    }

    pub fn object_expr(
        query: Option<GenExpr<QueryEffects>>,
        func: Option<(Pattern, Expr)>,
    ) -> Self {
        GenExpr::Object(ObjectExpr {
            query: query.map(|body| QueryExpr {
                body: Box::new(body),
            }),
            func: func.map(|(binding, body)| FuncExpr {
                binding,
                body: Box::new(body),
            }),
        })
    }
}
