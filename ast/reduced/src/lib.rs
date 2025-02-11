//! The Reduced Abstract Syntax Tree is a subset of [`sappho-ast`] for evaluation

mod expr;
mod pattern;
mod provider;

pub use sappho_ast_core::{Identifier, Literal};

pub use crate::expr::Expr;
pub use crate::pattern::{Pattern, UnpackPattern};
pub use crate::provider::AstRed;
