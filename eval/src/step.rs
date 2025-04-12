use derive_more::From;

#[derive(Debug, From)]
pub(crate) enum EvalStep<'s, FX> {
    /// This continuation concluded and produced a value
    #[from(Value, PrimVal)]
    Produce(Value),
    /// Evaluate `X` and send the result to `C`
    Continue(&'s Expr<FX>, ContExpr<'s, FX>),
}
