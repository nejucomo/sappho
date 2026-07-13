use derive_new::new;

use crate::outcome::ExecOutcome;

#[derive(Debug, new)]
pub(crate) struct Interaction {
    ix: usize,
    input: String,
    outcome: ExecOutcome,
}

impl std::fmt::Display for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { ix, input, outcome } = self;

        writeln!(f, "input {ix}: {input}")?;
        match outcome {
            Ok(v) => writeln!(f, "value {ix} -> {v}")?,
            Err(e) => writeln!(f, "<! error {ix} !> {e}")?,
        }
        writeln!(f)?;
        Ok(())
    }
}
