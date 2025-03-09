#[macro_export]
macro_rules! from_via_leftassoc_left {
    ( $s:ty => $t:ty ) => {
        impl From<$s> for $t {
            fn from(value: $s) -> Self {
                Self::from(LeftAssoc::from_left(value))
            }
        }
    };
}
