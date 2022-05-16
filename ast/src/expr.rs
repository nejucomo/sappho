//! Top-level expression type `GenExpr`, generic over [crate::effects]

use crate::{
    CommonExpr, FuncDef, ObjectDef, Pattern, PureExpr, QueryDef, RecursiveExpr, UniversalExpr,
};

#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Universal(UniversalExpr),
    Common(CommonExpr),
    Recursive(RecursiveExpr<Effects>),
    Effect(Effects),
}
