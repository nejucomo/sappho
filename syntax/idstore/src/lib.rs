mod arcid;
mod idstore;
mod resolve;

pub use crate::arcid::ArcId;
pub use crate::resolve::{resolve, resolve_static};

#[cfg(test)]
mod tests;
