//! This Abstract Syntax Tree corresponds to the textual grammar of `sappho`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation, which is embodied in the
//! `sappho-ast-reduced` crate. For example:
//!
//! `fn x -> x` is AST short-hand for reduced `{ fn x -> x }`.
//!
//! The top-level expression for evaluation is [PureExpr], which is a type alias to a general
//! expression type over all effects, [Expr]. The three bespoke effects are
//! [PureEffect], [QueryEffect], and [ProcEffect].

mod expr;
mod listexpr;
mod pattern;
mod provider;

pub use sappho_ast_core::{Effect, Identifier, Literal, ProcEffect, PureEffect, QueryEffect};

pub use self::expr::Expr;
pub use self::listexpr::ListExpr;
pub use self::pattern::{ListPattern, Pattern, UnpackPattern};
pub use self::provider::Ast;

pub type PureExpr = Expr<PureEffect>;
pub type QueryExpr = Expr<QueryEffect>;
pub type ProcExpr = Expr<ProcEffect>;
