use crate::Kernel;

/// Each extension grammar provides a [Root] expression
pub trait Root: From<Kernel<Self>> {}
