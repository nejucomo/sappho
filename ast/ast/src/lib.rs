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
mod pattern;
mod provider;

use sappho_ast_core::{CmtExpr, Literal};
use sappho_ast_effect::{ProcEffect, PureEffect, QueryEffect};
use sappho_listform::ListForm;

pub use self::expr::Expr;
pub use self::pattern::{ListPattern, Pattern};
pub use self::provider::Ast;

pub type PureExpr = CmtExpr<Ast, PureEffect>;
pub type QueryExpr = CmtExpr<Ast, QueryEffect>;
pub type ProcExpr = CmtExpr<Ast, ProcEffect>;

pub type ListExpr<FX> = ListForm<CmtExpr<Ast, FX>, CmtExpr<Ast, FX>>;
