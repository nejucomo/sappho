use crate::{cmds, Command, Options, Result};

pub trait RunCommand {
    fn cmd_run(&self, options: &Options) -> Result<'_, ()>;
}

impl RunCommand for Options {
    fn cmd_run(&self, options: &Options) -> Result<'_, ()> {
        self.command
            .as_ref()
            .unwrap_or(&Command::Repl)
            .cmd_run(options)
    }
}

impl RunCommand for Command {
    fn cmd_run(&self, _options: &Options) -> Result<'_, ()> {
        use Command::*;

        match self {
            Repl => cmds::repl(),
            Eval(opts) => cmds::eval(&opts.source),
            Parse(opts) => cmds::parse(&opts.source.source, &opts.format),
            Fuzz(opts) => cmds::fuzz(opts.max_depth, &opts.format),
        }
    }
}
