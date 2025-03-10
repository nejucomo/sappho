use sappho_unparse::Unparse;

/// The concrete expression grammar in which the [Kernel](crate::Kernel) grammar is embedded
pub trait Expression: Unparse + std::fmt::Debug {
    /// The concete pattern grammer in which [Pattern](crate::Pattern) is embedded
    type Pattern: Unparse + std::fmt::Debug;
}
