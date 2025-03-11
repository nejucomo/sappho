/// Components for error messages or other syntactic description contexts
#[derive(Copy, Clone, Debug)]
pub struct EffectDescription {
    /// The symbol prefix used to express this effect in an expression
    pub sigil: &'static str,

    /// The noun name of this effect
    pub noun: &'static str,

    /// The infinitive verb which fulfills "the expression `<symbol>x` will <infinitive> the value, `x`"
    pub verb: &'static str,
}
