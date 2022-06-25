pub use log::SetLoggerError;

pub fn init() -> Result<(), SetLoggerError> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Trace)
        .init()?;
    log::debug!("Log initialized.");
    Ok(())
}
