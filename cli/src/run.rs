use crate::{log, Options};

pub fn run() {
    let opt = Options::parse();
    log::init().unwrap();
    if let Some(e) = opt.run().err() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
