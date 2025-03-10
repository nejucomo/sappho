//! The kernel expression language with effects
//!
//! Consumer code should use one of the concrete extensions of [Kernel], either `RedExpr` or `RichExpr` found in dependant crates.
//!
//! # Naming Conventions
//!
//! Many types here represent a particular subexpression, but they have `Expr` in the name only when the name without `Expr` would be ambiguous.
//!
//! One case is with the effect kinds: "Pure", "Query", and "Proc"
//!
//! If we imagine `E` is any of these kinds, then:
//! - `<E>Expr`: an expression which allows `E` or stricter effects.
//! - `<E>Def`: is a pure expression or object clause that defines a computation in that effect. These are `query { ... }` and `proc { ... }`. There is no `"PureDef"`, since all expressions are ways to define a computation to produce a value.
//! - Each effect kind also has a unique verb and noun form for the effect itself:
//!   - Pure - "evaluate" / "evaluation"
//!   - Query - "inquire" / "inquiry"
//!   - Proc - "invoke" / "invocation"
//!
//! We use noun forms for type and discriminant names, and verb forms for functions which computes the effect.
mod application;
mod expr;
mod funcdef;
mod kernel;
pub mod r#let;
mod objectdef;
mod pattern;
pub mod proc;
pub mod query;
mod recursion;

pub use crate::application::Application;
pub use crate::expr::Expression;
pub use crate::funcdef::FuncDef;
pub use crate::kernel::Kernel;
pub use crate::objectdef::ObjectDef;
pub use crate::pattern::Pattern;
pub use crate::recursion::Recursion;
