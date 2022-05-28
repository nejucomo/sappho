use crate::{Options, Result};

pub fn run() -> Result<()> {
    Options::parse().run()
}
