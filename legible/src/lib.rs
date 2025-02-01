//! The [Legible] trait provides a flexible means of displaying hierarchically indented textual data which is displayed compactly where possible
#![deny(missing_docs)]

mod envelope;
mod fmtpos;
mod indentation;
mod intonode;
mod joint;
mod ldisp;
mod legible;
mod node;
mod position;
mod sequence;
mod stream;
mod trial;
mod wrappable;

pub use self::envelope::Envelope;
pub use self::intonode::IntoNode;
pub use self::joint::Joint;
pub use self::legible::Legible;
pub use self::node::Node;
pub use self::sequence::Sequence;

/// This is the default width threshold used in `fmt_with
pub const DEFAULT_FMT_WIDTH_THRESHOLD: usize = 80;
