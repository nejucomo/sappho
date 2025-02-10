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

pub use self::expr::Expr;
pub use self::pattern::{ListPattern, Pattern, UnpackPattern};
pub use self::provider::AstProvider;
pub use sappho_ast_core::{Identifier, Literal, ProcEffect, PureEffect, QueryEffect};

pub type ProcExpr = Expr<sappho_ast_core::ProcEffect>;
pub type PureExpr = Expr<sappho_ast_core::PureEffect>;
pub type QueryExpr = Expr<sappho_ast_core::QueryEffect>;

pub type ApplicationExpr<FX> = sappho_ast_core::ApplicationExpr<AstProvider, FX>;
pub type CoreExpr<FX> = sappho_ast_core::CoreExpr<AstProvider, FX>;
pub type EffectExpr<FX> = sappho_ast_core::EffectExpr<AstProvider, FX>;
pub type FuncDef = sappho_ast_core::FuncDef<AstProvider>;
pub type LetClause<FX> = sappho_ast_core::LetClause<AstProvider, FX>;
pub type LetExpr<FX> = sappho_ast_core::LetExpr<AstProvider, FX>;
pub type ListExpr<FX> = sappho_listform::ListForm<Expr<FX>, Box<Expr<FX>>>;
pub type LookupExpr<FX> = sappho_ast_core::LookupExpr<AstProvider, FX>;
pub type MatchClause<FX> = sappho_ast_core::MatchClause<AstProvider, FX>;
pub type MatchExpr<FX> = sappho_ast_core::MatchExpr<AstProvider, FX>;
pub type ObjectDef<FX> = sappho_ast_core::ObjectDef<AstProvider, FX>;
pub type ProcDef = sappho_ast_core::ProcDef<AstProvider>;
pub type QueryDef = sappho_ast_core::QueryDef<AstProvider>;
pub type Statements = sappho_ast_core::Statements<AstProvider>;
