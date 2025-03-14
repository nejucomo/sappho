mod runcmd;

use clap::{ArgEnum, Parser, Subcommand};
use sappho_source::Source;

use crate::Result;

/// sappho interpreter
#[derive(Debug, Parser)]
#[clap()]
pub struct Options {
    #[clap(flatten)]
    pub glopts: GlobalOptions,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub struct GlobalOptions {
    /// Turn on trace output
    #[clap(short, long)]
    pub trace: bool,
}

impl Options {
    pub fn parse() -> Self {
        <Options as Parser>::parse()
    }

    pub fn run(self) -> Result<()> {
        use self::runcmd::RunCommand;

        self.command.cmd_run(self.glopts)
    }
}

/// subcommands
#[derive(Debug, Subcommand)]
#[clap()]
pub enum Command {
    /// Eval an input
    #[clap()]
    Eval(SourceOptions),

    /// Parse an input
    #[clap()]
    Parse(ParseOptions),

    /// Generate a random expression
    #[clap()]
    Fuzz(FuzzOptions),
}

/// source options
#[derive(Debug, Parser)]
#[clap()]
pub struct SourceOptions {
    #[clap(default_value_t)]
    source: Source,
}

/// parse options
#[derive(Debug, Parser)]
#[clap()]
pub struct ParseOptions {
    /// Select the parse output format
    #[clap(arg_enum, long, short, default_value = "canonical")]
    format: UnparseFormat,

    #[clap(flatten)]
    source: SourceOptions,
}

/// parse output formats
#[derive(ArgEnum, Clone, Debug)]
#[clap()]
pub enum UnparseFormat {
    /// The internal AST representation
    AST,

    /// Direct unparse
    Direct,

    /// The canonicalized source code
    Canonical,

    /// The reduced source code
    Reduced,
}

/// fuzz options
#[derive(Debug, Parser)]
#[clap()]
pub struct FuzzOptions {
    /// Select the parse output format
    #[clap(arg_enum, long, short, default_value = "canonical")]
    format: UnparseFormat,

    /// The max recursion depth
    #[clap(long, short, default_value = "3")]
    max_depth: usize,
}
