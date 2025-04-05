use derive_more::From;

/// An integer-index-based reference
#[derive(Copy, Clone, Debug, From)]
pub struct IxRef(pub(crate) usize);
