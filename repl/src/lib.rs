#[deny(unsafe_code)]
mod error;
mod interaction;
mod outcome;
mod repl;

pub use self::error::{ReplError, ReplResult};

pub fn run_repl() -> ReplResult<()> {
    repl::Repl::default().run()
}
