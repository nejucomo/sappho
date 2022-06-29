mod application;
mod literal;

pub use self::application::ApplicationExpr;
pub use self::literal::Literal;

/// An identifier such as the name of the argument and reference in `fn x -> x`.
pub type Identifier = sappho_identmap::Identifier;
