use derive_new::new;

use crate::outcome::ExecOutcome;

#[derive(Debug, new)]
pub(crate) struct Interaction {
    ix: usize,
    input: String,
    outcome: ExecOutcome,
}

impl Interaction {
    pub(crate) fn outcome_string(&self) -> String {
        let Self { ix, outcome, .. } = self;
        match outcome {
            Ok(v) => format!("value {ix} -> {v}\n"),
            Err(e) => format!("!ERROR {ix}!\n{e}\n"),
        }
    }
}

impl std::fmt::Display for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { ix, input, .. } = self;
        let input = input.trim_end();
        let ocstr = self.outcome_string();

        writeln!(f, "input {ix}: {input}\n{ocstr}")
    }
}
