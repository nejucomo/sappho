//! The [Legible] trait provides a flexible means of displaying hierarchically indented textual data which is displayed compactly where possible
#![deny(missing_docs)]

mod bracketed;
mod headandtail;
mod indentation;
mod innernode;
mod intonode;
mod joint;
mod keyvalue;
mod ldisp;
mod legible;
mod node;
mod position;
mod separatedseq;
mod stream;
mod text;
mod trial;
mod wrappable;
mod writestr;

pub use self::bracketed::BracketSeq;
pub use self::headandtail::HeadAndTail;
pub use self::intonode::IntoNode;
pub use self::keyvalue::KeyValue;
pub use self::legible::Legible;
pub use self::node::Node;
pub use self::separatedseq::SeparatedSeq;
pub use self::text::Text;

/// This is the default width threshold used in `fmt_with
pub const DEFAULT_FMT_WIDTH_THRESHOLD: usize = 80;
