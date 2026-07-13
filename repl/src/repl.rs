use sappho_code_origin::CodeOrigin;
use sappho_eval::eval;
use sappho_parser::parse;
use sappho_transform::reduce;

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
        let origin = format!("input {ix}");
        let input = self.prompt_for_input(&origin)?;
        let outcome = self.exec(CodeOrigin::new(&input, &origin));
        let ntx = Interaction::new(ix, input, outcome);
        self.print(ntx.to_string())?;
        self.history.push(ntx);
        Ok(())
    }

    fn prompt_for_input(&mut self, label: &str) -> ReplResult<String> {
        self.print(format!("{label}: "))?;

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

    fn exec(&mut self, code: CodeOrigin<'_>) -> ExecOutcome {
        let expr = parse(code)?;
        let val = eval(reduce(expr))?;
        Ok(val)
    }
}
