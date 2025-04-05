use derive_more::From;

/// A [usize]-based scoped reference
#[derive(Copy, Clone, Debug, PartialEq, From)]
pub struct IxRef(usize);
