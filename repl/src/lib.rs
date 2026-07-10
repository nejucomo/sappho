#[deny(unsafe_code)]
mod error;
mod repl;

pub use self::error::{ReplError, ReplResult};

pub fn run_repl() -> ReplResult<()> {
    repl::Repl::default().run()
}
