//! The Reduced Abstract Syntax Tree is a subset of [`sappho-ast`] for evaluation

mod expr;
mod lfreduce;
mod pattern;
mod provider;

use sappho_ast::{ProcEffect, PureEffect, QueryEffect};
pub use sappho_ast_core::{Identifier, Literal};

pub use crate::expr::Expr;
pub use crate::pattern::{Pattern, UnpackPattern};
pub use crate::provider::AstRed;

pub type PureExpr = Expr<PureEffect>;
pub type QueryExpr = Expr<QueryEffect>;
pub type ProcExpr = Expr<ProcEffect>;
