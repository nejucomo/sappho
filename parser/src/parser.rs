mod universal;

use crate::delimited::delimited;
use crate::space::ws;
use crate::Error;
use chumsky::primitive::just;
use chumsky::recursive::{recursive, Recursive};
use chumsky::text;
use chumsky::Parser;
use saplang_ast::{GenExpr, Pattern, PureEffects, PureExpr, QueryEffects};

pub(crate) fn expression() -> impl Parser<char, PureExpr, Error = Error> {
    recursive(expr).then_ignore(chumsky::primitive::end())
}

trait FxParser: Sized + 'static {
    fn fx_parser() -> Box<dyn Parser<char, Self, Error = Error>>;
}

fn expr<FX>(
    expr: Recursive<'_, char, GenExpr<FX>, Error>,
) -> impl Parser<char, GenExpr<FX>, Error = Error> + '_
where
    FX: FxParser,
{
    let inner = parens_expr(expr.clone())
        .or(self::universal::universal().map(GenExpr::Universal))
        .or(object_expr())
        .or(let_expr(expr.clone()))
        .or(func_expr())
        .or(list(expr))
        .or(FX::fx_parser().map(GenExpr::Effect));

    let innerws = inner.then_ignore(ws().or_not());

    innerws.repeated().at_least(1).map(|exprs| {
        exprs
            .into_iter()
            .reduce(GenExpr::application)
            .expect(".at_least(1) postcondition failed.")
    })
}

impl FxParser for PureEffects {
    fn fx_parser() -> Box<dyn Parser<char, Self, Error = Error>> {
        Box::new(
            chumsky::primitive::empty()
                .try_map(|(), span| Err(Error::custom(span, "<internal non-parsing case>"))),
        )
    }
}

impl FxParser for QueryEffects {
    fn fx_parser() -> Box<dyn Parser<char, Self, Error = Error>> {
        use QueryEffects::Inquire;

        Box::new(
            just('$')
                .then_ignore(ws().or_not())
                .ignore_then(recursive(expr))
                .map(|qx| Inquire(Box::new(qx))),
        )
    }
}

fn parens_expr<FX>(
    expr: Recursive<'_, char, GenExpr<FX>, Error>,
) -> impl Parser<char, GenExpr<FX>, Error = Error> + '_
where
    FX: FxParser,
{
    delimited('(', expr, ')')
}

fn list<FX>(
    expr: Recursive<'_, char, GenExpr<FX>, Error>,
) -> impl Parser<char, GenExpr<FX>, Error = Error> + '_
where
    FX: FxParser,
{
    use crate::listform::list_form;

    list_form(expr).map(GenExpr::list)
}

fn let_expr<FX>(
    expr: Recursive<'_, char, GenExpr<FX>, Error>,
) -> impl Parser<char, GenExpr<FX>, Error = Error> + '_
where
    FX: FxParser,
{
    text::keyword("let")
        .then_ignore(ws())
        .ignore_then(pattern())
        .then_ignore(just('=').delimited_by(ws(), ws()))
        .then(expr.clone())
        .then_ignore(just(';'))
        .then_ignore(ws())
        .then(expr)
        .map(|((binding, bindexpr), tail)| GenExpr::let_expr(binding, bindexpr, tail))
}

fn func_expr<FX>() -> impl Parser<char, GenExpr<FX>, Error = Error> {
    func_clause().map(GenExpr::func_expr)
}

fn func_clause() -> impl Parser<char, (Pattern, PureExpr), Error = Error> {
    text::keyword("fn")
        .then_ignore(ws())
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(recursive(expr))
}

fn object_expr<FX>() -> impl Parser<char, GenExpr<FX>, Error = Error> {
    delimited('{', func_clause().or_not(), '}').map(|fe| GenExpr::object_expr(None, fe))
}

fn pattern() -> impl Parser<char, Pattern, Error = Error> {
    text::ident()
}
