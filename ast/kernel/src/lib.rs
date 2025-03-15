//! The [Kernel] is the common subset of the AST shared by both the rich and reduced forms
mod application;
mod boxwise;
mod funcdef;
mod interaction;
mod kernel;
mod letexpr;
mod literal;
mod lookup;
mod matchexpr;
mod objectdef;
mod proc;
mod provider;
mod querydef;
mod wise;

use sappho_ast_effect::{ProcEffect, PureEffect, QueryEffect};

// Top-level Aliases:
pub type ProcExpr<XP> = BoxWise<XP, ProcEffect>;
pub type QueryExpr<XP> = BoxWise<XP, QueryEffect>;
pub type PureExpr<XP> = BoxWise<XP, PureEffect>;

pub use self::application::ApplicationExpr;
pub use self::boxwise::BoxWise;
pub use self::funcdef::FuncDef;
pub use self::interaction::Interaction;
pub use self::kernel::Kernel;
pub use self::letexpr::{LetClause, LetExpr};
pub use self::literal::Literal;
pub use self::lookup::LookupExpr;
pub use self::matchexpr::{MatchClause, MatchExpr};
pub use self::objectdef::ObjectDef;
pub use self::proc::{ProcDef, Statements};
pub use self::provider::AstProvider;
pub use self::querydef::QueryDef;
pub use self::wise::Wise;
