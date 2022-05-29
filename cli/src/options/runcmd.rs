use crate::{cmds, Command, Options, ParseOptions, Result};

pub trait RunCommand {
    fn cmd_run(&self, options: &Options) -> Result<()>;
}

impl RunCommand for Options {
    fn cmd_run(&self, options: &Options) -> Result<()> {
        self.command.cmd_run(options)
    }
}

impl RunCommand for Command {
    fn cmd_run(&self, options: &Options) -> Result<()> {
        use Command::*;

        match self {
            Parse(opts) => opts.cmd_run(options),
        }
    }
}

impl RunCommand for ParseOptions {
    fn cmd_run(&self, _options: &Options) -> Result<()> {
        let source = self.source.read()?;
        cmds::parse(self.source.path(), &source)
    }
}
