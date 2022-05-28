mod runcmd;

use crate::{Result, SourceOption};
use clap::{Parser, Subcommand};

/// sappho interpreter
#[derive(Debug, Parser)]
#[clap()]
pub struct Options {
    #[clap(subcommand)]
    command: Command,
}

impl Options {
    pub fn parse() -> Self {
        <Options as Parser>::parse()
    }

    pub fn run(&self) -> Result<()> {
        use self::runcmd::RunCommand;

        self.cmd_run(self)
    }
}

/// subcommands
#[derive(Debug, Subcommand)]
#[clap()]
pub enum Command {
    /// Parse an input
    #[clap()]
    Parse(ParseOptions),
}

/// parse options
#[derive(Debug, Parser)]
#[clap()]
pub struct ParseOptions {
    #[clap(default_value_t)]
    source: SourceOption,
}
