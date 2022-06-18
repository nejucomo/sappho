use crate::Options;

pub fn run() {
    let opt = Options::parse();
    if let Some(e) = opt.run().err() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
