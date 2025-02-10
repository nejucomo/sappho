mod runcmd;

use crate::{Result, SourceOption};
use clap::{ArgEnum, Parser, Subcommand};

/// sappho interpreter
#[derive(Debug, Parser)]
#[clap()]
pub struct Options {
    /// Turn on trace output
    #[clap(short, long)]
    pub trace: bool,

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
    source: SourceOption,
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
