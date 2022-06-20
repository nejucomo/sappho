//! Common sub-grammars for all effects

mod func;
mod object;
mod query;

pub use self::func::FuncDef;
pub use self::object::ObjectDef;
pub use self::query::QueryDef;
