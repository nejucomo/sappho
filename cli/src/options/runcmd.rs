use crate::options::GlobalOptions;
use crate::{cmds, Command, Result};

pub trait RunCommand {
    fn cmd_run(self, glopts: GlobalOptions) -> Result<()>;
}

impl RunCommand for Command {
    fn cmd_run(self, _: GlobalOptions) -> Result<()> {
        use Command::*;

        match self {
            Eval(opts) => cmds::eval(opts.source),
            Parse(opts) => cmds::parse(opts.source.source, opts.format),
            Fuzz(opts) => cmds::fuzz(opts.max_depth, opts.format),
        }
    }
}
