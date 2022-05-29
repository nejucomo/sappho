use crate::{Options, Result};

pub fn run() {
    if let Some(e) = run_result().err() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run_result() -> Result<()> {
    Options::parse().run()
}
