mod runcmd;

use crate::{Result, SourceOption};
use clap::{ArgEnum, Parser, Subcommand};

/// sappho interpreter
#[derive(Debug, Parser)]
pub struct Options {
    /// Turn on trace output
    #[clap(short, long)]
    pub trace: bool,

    #[clap(subcommand)]
    command: Option<Command>,
}

impl Options {
    pub fn parse() -> Self {
        <Options as Parser>::parse()
    }

    pub fn run(&self) -> Result<'_, ()> {
        use self::runcmd::RunCommand;

        self.cmd_run(self)
    }
}

/// subcommands
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start the interactive REPL (default command)
    Repl,

    /// Eval an input
    Eval(SourceOptions),

    /// Parse an input
    Parse(ParseOptions),

    /// Generate a random expression
    Fuzz(FuzzOptions),
}

/// source options
#[derive(Debug, Parser)]
pub struct SourceOptions {
    #[clap(default_value_t)]
    source: SourceOption,
}

/// parse options
#[derive(Debug, Parser)]
pub struct ParseOptions {
    /// Select the parse output format
    #[clap(arg_enum, long, short, default_value = "canonical")]
    format: UnparseFormat,

    #[clap(flatten)]
    source: SourceOptions,
}

/// parse output formats
#[derive(ArgEnum, Clone, Debug)]
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
pub struct FuzzOptions {
    /// Select the parse output format
    #[clap(arg_enum, long, short, default_value = "canonical")]
    format: UnparseFormat,

    /// The max recursion depth
    #[clap(long, short, default_value = "3")]
    max_depth: usize,
}
