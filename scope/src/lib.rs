//! Index-based runtime scope

#![deny(missing_docs, unsafe_code)]

mod ixref;
mod rtmd;
mod scope;

pub use crate::ixref::IxRef;
pub use crate::rtmd::RtMetadata;
pub use crate::scope::Scope;
