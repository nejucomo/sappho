use crate::Wise;

/// Boxed-Spanned-Expression
#[derive(Debug)]
pub struct BoxWise<FX>(Box<Wise<FX>>);
