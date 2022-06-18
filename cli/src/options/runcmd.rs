use crate::{cmds, Command, Options, Result};

pub trait RunCommand {
    fn cmd_run(&self, options: &Options) -> Result<()>;
}

impl RunCommand for Options {
    fn cmd_run(&self, options: &Options) -> Result<()> {
        self.command.cmd_run(options)
    }
}

impl RunCommand for Command {
    fn cmd_run(&self, _options: &Options) -> Result<()> {
        use Command::*;

        match self {
            Eval(opts) => cmds::eval(&opts.source),
            Parse(opts) => cmds::parse(&opts.source),
            Canonicalize(opts) => cmds::canonicalize(&opts.source),
        }
    }
}
