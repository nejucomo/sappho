//! This Abstract Syntax Tree corresponds to the textual grammar of `sappho`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation. Example:
//!
//! `fn x -> x` is short-hand for `{ fn x -> x }`.

mod common;
mod effects;
mod expr;
mod listform;
mod literal;
mod recursive;

pub type Pattern = Identifier;

pub use self::common::{FuncDef, ObjectDef, QueryDef};
pub use self::effects::{ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr};
pub use self::expr::GenExpr;
pub use self::listform::ListForm;
pub use self::literal::Literal;
pub use self::recursive::{Application, LetExpr, Lookup};
pub use sappho_identmap::Identifier;
