//! This Abstract Syntax Tree corresponds to the textual grammar of `sappho`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation. Example:
//!
//! `fn x -> x` is short-hand for `{ fn x -> x }`.

mod common;
mod effects;
mod expr;
mod listform;
mod recursive;
mod universal;

pub use self::common::{CommonExpr, FuncDef, ObjectDef, QueryDef};
pub use self::effects::{ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr};
pub use self::expr::GenExpr;
pub use self::listform::ListForm;
pub use self::recursive::{Application, LetExpr, Lookup, RecursiveExpr};
pub use self::universal::{Literal, Pattern, UniversalExpr};
pub use sappho_identmap::Identifier;
