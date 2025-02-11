//! The Reduced Abstract Syntax Tree is a subset of [`sappho-ast`] for evaluation

mod expr;
mod pattern;
mod provider;

pub use sappho_ast_core::{Identifier, Literal};

pub use crate::expr::Expr;
pub use crate::pattern::{Pattern, UnpackPattern};
pub use crate::provider::AstReduced;

pub type ProcExpr = Expr<sappho_ast_core::ProcEffect>;
pub type PureExpr = Expr<sappho_ast_core::PureEffect>;
pub type QueryExpr = Expr<sappho_ast_core::QueryEffect>;

pub type ApplicationExpr<FX> = sappho_ast_core::ApplicationExpr<AstReduced, FX>;
pub type CoreExpr<FX> = sappho_ast_core::CoreExpr<AstReduced, FX>;
pub type EffectExpr<FX> = sappho_ast_core::EffectExpr<AstReduced, FX>;
pub type FuncClause = sappho_ast_core::FuncDef<AstReduced>;
pub type LetClause<FX> = sappho_ast_core::LetClause<AstReduced, FX>;
pub type LetExpr<FX> = sappho_ast_core::LetExpr<AstReduced, FX>;
pub type LookupExpr<FX> = sappho_ast_core::LookupExpr<AstReduced, FX>;
pub type MatchClause<FX> = sappho_ast_core::MatchClause<AstReduced, FX>;
pub type MatchExpr<FX> = sappho_ast_core::MatchExpr<AstReduced, FX>;
pub type ObjectDef<FX> = sappho_ast_core::ObjectDef<AstReduced, FX>;
pub type ProcClause = sappho_ast_core::ProcDef<AstReduced>;
pub type QueryClause = sappho_ast_core::QueryDef<AstReduced>;
