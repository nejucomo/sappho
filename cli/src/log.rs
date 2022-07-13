pub use log::SetLoggerError;

pub fn init(trace: bool) -> Result<(), SetLoggerError> {
    use log::LevelFilter::{Info, Trace};
    use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode::Stderr};

    TermLogger::init(
        if trace { Trace } else { Info },
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
