pub use log::SetLoggerError;

pub fn init() -> Result<(), SetLoggerError> {
    use log::LevelFilter::Trace;
    use simplelog::{ColorChoice, Config, TermLogger, TerminalMode::Stderr};

    TermLogger::init(Trace, Config::default(), Stderr, ColorChoice::Auto)?;

    log::debug!("Log initialized.");
    Ok(())
}
