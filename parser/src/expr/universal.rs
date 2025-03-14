use chumsky::Parser as _;
use sappho_ast_core::Literal;
use sappho_ast_rich::ProcExpr;
use sappho_identifier::RcId;
use sappho_parsable::{Parsable, Parser};
use sappho_primval::PrimVal;

pub(super) fn universal_expr() -> impl Parser<ProcExpr> {
    use sappho_ast_core::CoreExpr::{Lit, Ref};

    identifier()
        .map(Ref)
        .or(literal().map(Lit))
        .map(ProcExpr::from)
}

pub(super) fn identifier() -> impl Parser<RcId> {
    RcId::parser()
}

pub(super) fn literal() -> impl Parser<Literal> {
    PrimVal::parser().map(Literal::from)
}
