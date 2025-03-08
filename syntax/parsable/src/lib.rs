pub mod error;
mod intoparser;
mod parsable;
mod parser;
pub mod primitive;
mod with;

use crate::error::ChumskyError;

pub type Recursive<'r, O> = chumsky::recursive::Recursive<'r, char, O, ChumskyError>;

pub use crate::intoparser::IntoParser;
pub use crate::parsable::Parsable;
pub use crate::parser::Parser;
pub use crate::with::ParsableWith;
