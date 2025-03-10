use sappho_parsable::Parsable;

/// The concrete expression grammar in which the [Kernel](crate::Kernel) grammar is embedded
pub trait Expression: Parsable {
    /// The concete pattern grammer in which [Pattern](crate::Pattern) is embedded
    type Pattern: Parsable;
}
