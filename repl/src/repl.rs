use crate::interaction::Interaction;
use crate::outcome::ExecOutcome;
use crate::ReplResult;

#[derive(Debug, Default)]
pub(crate) struct Repl {
    history: Vec<Interaction>,
}

impl Repl {
    pub(crate) fn run(mut self) -> ReplResult<()> {
        loop {
            self.handle_one_interaction()?;
        }
    }

    fn handle_one_interaction(&mut self) -> ReplResult<()> {
        let ix = self.history.len();
        let input = self.prompt_for_input(ix)?;
        let outcome = self.exec(&input);
        let ntx = Interaction::new(ix, input, outcome);
        self.print(ntx.to_string())?;
        self.history.push(ntx);
        Ok(())
    }

    fn prompt_for_input(&mut self, ix: usize) -> ReplResult<String> {
        self.print(format!("input {ix}: "))?;

        let mut input = "".to_string();
        let n = std::io::stdin().read_line(&mut input)?;
        assert_eq!(n, input.len());

        Ok(input)
    }

    fn print(&mut self, msg: impl AsRef<[u8]>) -> ReplResult<()> {
        use std::io::Write as _;

        let mut out = std::io::stdout().lock();
        out.write_all(msg.as_ref())?;
        out.flush()?;
        Ok(())
    }

    fn exec(&mut self, input: &str) -> ExecOutcome {
        todo!("{input:?}")
    }
}
