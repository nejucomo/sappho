pub(crate) trait Eval<I, O> {
    fn eval(self, input: I) -> O;
}
