pub use log::SetLoggerError;

pub fn init() -> Result<(), SetLoggerError> {
    use log::LevelFilter::Trace;
    use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode::Stderr};

    TermLogger::init(
        Trace,
        ConfigBuilder::new()
            .set_time_format_rfc3339()
            .set_thread_level(Trace)
            .build(),
        Stderr,
        ColorChoice::Auto,
    )?;

    log::debug!("Log initialized.");
    Ok(())
}
