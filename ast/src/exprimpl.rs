use crate::{
    Application, Expr, FuncDef, GenExpr, LetExpr, ObjectDef, Pattern, QueryEffects, QueryDef,
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
        GenExpr::Func(FuncDef {
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
        GenExpr::Query(QueryDef {
            body: Box::new(body),
        })
    }

    pub fn object_expr(
        query: Option<GenExpr<QueryEffects>>,
        func: Option<(Pattern, Expr)>,
    ) -> Self {
        GenExpr::Object(ObjectDef {
            query: query.map(|body| QueryDef {
                body: Box::new(body),
            }),
            func: func.map(|(binding, body)| FuncDef {
                binding,
                body: Box::new(body),
            }),
        })
    }
}
